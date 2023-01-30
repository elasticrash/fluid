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
        plan: ActiveValue::Set(task.plan),
        start: ActiveValue::Set(Utc::now().naive_utc()),
        ..Default::default()
    };
    let _ = Schedule::insert(schedule).exec(db).await;

    status::Accepted(Some("Task Accepted".to_string()))
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
        .mount("/", routes![schedule_event])
}
