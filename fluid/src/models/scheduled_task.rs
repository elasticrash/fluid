use chrono::{NaiveDateTime, Utc};
use rocket::data::ToByteUnit;
use rocket::data::{Data, FromData, Outcome};
use rocket::http::Status;
use rocket::request::{self, Request};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Task {
    pub name: Option<String>,
    pub expression: String,
    pub start: Option<NaiveDateTime>,
    pub finish: Option<NaiveDateTime>,
    pub endpoint: String,
}

#[derive(Debug)]
pub enum MyError {
    PayloadTooLarge,
    NoBodyProvidedOrInvalidBody,
    InvalidExpression,
    Io(std::io::Error),
}

#[rocket::async_trait]
impl<'r> FromData<'r> for Task {
    type Error = MyError;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
        use rocket::outcome::Outcome::*;

        let string = match data.open(4.kilobytes()).into_string().await {
            Ok(string) if string.is_complete() => string.into_inner(),
            Ok(_) => return Failure((Status::PayloadTooLarge, MyError::PayloadTooLarge)),
            Err(e) => return Failure((Status::InternalServerError, MyError::Io(e))),
        };

        let string_body = request::local_cache!(req, string);
        let task = serde_json::from_str::<Task>(string_body);

        info!("{}", string_body);
        info!("{:?}", task);

        match task {
            Ok(mut tsk) => {
                if tsk.name.is_none() {
                    tsk.name = Some(format!("default-{}", chrono::offset::Utc::now()));
                }

                if tsk.start.is_none() {
                    tsk.start = Some(Utc::now().naive_utc());
                }
                let expression: Vec<&str> = tsk.expression.split(':').collect();
                if expression.len() == 2 && expression[0].parse::<i32>().is_ok() {
                    return Success(tsk);
                }
                return Failure((Status::BadRequest, MyError::InvalidExpression));
            }
            Err(_) => return Failure((Status::BadRequest, MyError::NoBodyProvidedOrInvalidBody)),
        }
    }
}
