mod models;

use chrono::Utc;
use fluid_common::config::read;
use fluid_db::configuration::Configuration;
use fluid_db::entities::{prelude::*, *};
use fluid_db::set_up_db;
use rocket::response::status;
use rocket::State;
use sea_orm::*;
use std::env;

use crate::models::scheduled_task::Task;

#[macro_use]
extern crate rocket;

#[post("/schedule", format = "application/json", data = "<task>")]
async fn schedule_event(db: &State<DatabaseConnection>, task: Task) -> status::Accepted<String> {
    let db = db as &DatabaseConnection;

    let schedule = schedule::ActiveModel {
        name: ActiveValue::Set(task.name.unwrap()),
        expression: ActiveValue::Set(task.expression),
        start: ActiveValue::Set(Utc::now().naive_utc()),
        finish: ActiveValue::Set(task.finish),
        ..Default::default()
    };
    let scheduled_process = Schedule::insert(schedule.clone()).exec(db).await;

    let call_back = call_back::ActiveModel {
        endpoint: ActiveValue::Set(task.endpoint),
        schedule_id: ActiveValue::Set(scheduled_process.unwrap().last_insert_id),
        ..Default::default()
    };

    let _call_back_process = CallBack::insert(call_back).exec(db).await;

    status::Accepted(Some("Task Accepted".to_string()))
}

#[get("/loop")]
async fn verify_callback() -> status::Accepted<String> {
    status::Accepted(Some("Event Received".to_string()))
}

#[launch]
async fn rocket() -> _ {
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

    rocket::build()
        .manage(db)
        .mount("/", routes![schedule_event, verify_callback])
}
