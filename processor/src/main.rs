use anyhow::Result;
use chrono::Utc;
use crossbeam_channel::{bounded, select, tick, Receiver};
use reqwest::Client;
use sea_orm::sea_query::Expr;
use std::env;
use uuid::Uuid;

use fluid_common::config::read;
use fluid_db::configuration::Configuration;
use fluid_db::entities::{prelude::*, *};
use fluid_db::set_up_db;
use sea_orm::*;

fn ctrl_channel() -> Result<Receiver<()>, ctrlc::Error> {
    let (sender, receiver) = bounded(100);
    ctrlc::set_handler(move || {
        let _ = sender.send(());
    })?;

    Ok(receiver)
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let config: Configuration = if args.len() > 1 && args[1] == "-f" {
        read(&args[2])
    } else {
        read("config.json")
    };

    let db = match set_up_db(&config).await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };

    let ctrl_c_events = ctrl_channel()?;
    let ticks = tick(std::time::Duration::from_secs(1));

    loop {
        select! {
            recv(ticks) -> _ => process_tasks(&db).await,
            recv(ctrl_c_events) -> _ => {
                println!();
                println!("Goodbye!");
                break;
            }
        }
    }

    Ok(())
}

async fn process_tasks(db: &DatabaseConnection) {
    let tasks: Vec<execution::Model> = match Execution::find()
        .filter(
            Condition::all()
                .add(execution::Column::When.lte(Utc::now()))
                .add(execution::Column::Pending.eq(1))
                .add(execution::Column::ClaimedBy.is_null()),
        )
        .all(db)
        .await
    {
        Ok(records) => records,
        Err(err) => {
            println!("Unable to perform a query at this point, due: {}", err);
            vec![]
        }
    };

    println!("found {} tasks", tasks.len());

    for task in tasks {
        let mut try_to_claim_task: execution::ActiveModel = task.clone().into();

        let affected_rows = match Execution::update_many()
            .col_expr(
                execution::Column::ClaimedBy,
                Expr::value(Some(Uuid::new_v4().to_string())),
            )
            .filter(
                Condition::all()
                    .add(execution::Column::ClaimedBy.is_null())
                    .add(execution::Column::Id.eq(try_to_claim_task.clone().id.unwrap())),
            )
            .exec(db)
            .await
        {
            Ok(data) => data.rows_affected,
            Err(err) => panic!("{}", err),
        };

        if affected_rows == 0 {
            println!(
                "record with id {}, was claimed by another instance",
                try_to_claim_task.clone().id.unwrap()
            );

            return;
        }

        let call_back: Vec<call_back::Model> = match CallBack::find()
            .filter(
                call_back::Column::ScheduleId.eq(try_to_claim_task.clone().schedule_id.unwrap()),
            )
            .all(db)
            .await
        {
            Ok(records) => records,
            Err(err) => {
                println!("Unable to perform a query at this point, due: {}", err);
                vec![]
            }
        };

        let client = Client::new();

        let call_back_clone = call_back[0].clone();
        let execution_clone = task.clone();

        match client
            .get(format!(
                "{}?name={}&time={}",
                call_back_clone.endpoint.to_owned(),
                execution_clone.name,
                execution_clone.when
            ))
            .send()
            .await
        {
            Ok(_) => println!(
                "even with id {} was send to {}",
                try_to_claim_task.clone().id.unwrap(),
                call_back[0].clone().endpoint,
            ),
            Err(err) => println!("Error communicating with webhook URL : {}", err),
        };
        try_to_claim_task.pending = ActiveValue::Set(0);

        if let Err(err) = try_to_claim_task.update(db).await {
            println!("failed to update record : {}", err)
        };
    }
}
