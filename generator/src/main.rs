use anyhow::Result;
use chrono::{Duration, NaiveDateTime, Utc};
use crossbeam_channel::{bounded, select, tick, Receiver};
use std::env;

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
            recv(ticks) -> _ => create_tasks(&db).await,
            recv(ctrl_c_events) -> _ => {
                println!();
                println!("Goodbye!");
                break;
            }
        }
    }

    Ok(())
}

async fn create_tasks(db: &DatabaseConnection) {
    let events: Vec<schedule::Model> = match Schedule::find()
        .filter(
            Condition::any()
                .add(schedule::Column::Process.is_null())
                .add(schedule::Column::Process.lte(Utc::now()))
                .add(schedule::Column::Finish.gte(Utc::now())),
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

    println!("found {:?} records", events.len());

    for e in events {
        let expression: Vec<&str> = e.expression.split(':').collect();
        let time_value = expression[0].parse::<i64>().unwrap();

        let mut last_insert: Option<NaiveDateTime> = None;
        let start = if e.process.is_some() {
            e.process.unwrap()
        } else {
            e.start
        };

        for i in 1..11 {
            let execution = execution::ActiveModel {
                schedule_id: ActiveValue::Set(e.id),
                name: ActiveValue::Set(e.clone().name),
                pending: ActiveValue::Set(1),
                when: match expression[1] {
                    "Y" => ActiveValue::Set(start + Duration::days(time_value * 365 * i)),
                    "M" => ActiveValue::Set(start + Duration::days(time_value * 30 * i)),
                    "D" => ActiveValue::Set(start + Duration::days(time_value * i)),
                    "h" => ActiveValue::Set(start + Duration::hours(time_value * i)),
                    "m" => ActiveValue::Set(start + Duration::minutes(time_value * i)),
                    "s" => ActiveValue::Set(start + Duration::seconds(time_value * i)),
                    _ => panic!(),
                },
                ..Default::default()
            };

            last_insert = Some(execution.clone().when.unwrap());
            if e.finish.is_some() && execution.clone().when.unwrap() > e.clone().finish.unwrap() {
                break;
            }
            if let Err(err) = Execution::insert(execution).exec(db).await {
                println!("failed to write record : {}", err)
            };
        }
        if last_insert.is_some() {
            let mut ev: schedule::ActiveModel = e.clone().into();
            ev.process = Set(last_insert.to_owned());
            if let Err(err) = ev.update(db).await {
                println!("failed to update record : {}", err)
            };
        }
    }
}
