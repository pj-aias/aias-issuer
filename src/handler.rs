use actix_session::Session;
use actix_web::Error as WebError;
use actix_web::{web, HttpResponse, Responder};

use crate::utils;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::env;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct BasicResponse {}

#[derive(Deserialize, Serialize)]
pub struct PrepareSMSAuthReq {
    pub phone_number: String,
}

pub async fn hello() -> impl Responder {
    println!("hello");
    HttpResponse::Ok().body("Hello world")
}

pub async fn prepare_sms_auth(
    phone_number: web::Json<PrepareSMSAuthReq>,
    session: Session,
) -> Result<HttpResponse, WebError> {
    println!("send_sms");

    let is_debugging = env::var("DEBUGGING").expect("Find DEBUGGING environment variable");
    let token: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    let phone_number = &phone_number.phone_number;
    let body = format!("token {}", token);

    if is_debugging == "true" {
        match utils::send_sms(phone_number, &body).await {
            Ok(_) => {}
            Err(_) => return utils::get_err_resp().await,
        }
    }

    match session.set("phone_number", phone_number) {
        Ok(_) => {}
        Err(_) => return utils::get_err_resp().await,
    }

    match session.set("token", token) {
        Ok(_) => {}
        Err(_) => return utils::get_err_resp().await,
    }

    HttpResponse::Ok().json(BasicResponse {}).await
}
