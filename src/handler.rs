use actix_session::Session;
use actix_web::Error as WebError;
use actix_web::{web, HttpResponse, Responder};

use crate::utils;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::env;

use crate::db;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct BasicResponse {}

#[derive(Deserialize, Serialize)]
pub struct SendCodeReq {
    pub phone_number: String,
}

#[derive(Deserialize, Serialize)]
pub struct VerifyCodeReq {
    pub code: String,
}

#[derive(Deserialize, Serialize)]
pub struct VerifyCodeResp {
    pub token: String,
}

pub async fn hello() -> impl Responder {
    println!("hello");
    HttpResponse::Ok().body("Hello world")
}

pub async fn send_code(
    phone_number: web::Json<SendCodeReq>,
    session: Session,
) -> Result<HttpResponse, WebError> {
    println!("send_sms");

    let is_debugging = env::var("AIAS_DEBUG").expect("Find DEBUGGING environment variable");
    let code: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    let phone_number = &phone_number.phone_number;
    let body = format!("code {}", code);

    if is_debugging == "true" {
        env::set_var("AIAS_TEST_CODE", code.clone());
    } else {
        match utils::send_sms(phone_number, &body).await {
            Ok(_) => {}
            Err(_) => return utils::get_err_resp().await,
        }
    }

    match session.set("phone_number", phone_number) {
        Ok(_) => {}
        Err(_) => return utils::get_err_resp().await,
    }

    match session.set("code", code) {
        Ok(_) => {}
        Err(_) => return utils::get_err_resp().await,
    }

    HttpResponse::Ok().json(BasicResponse {}).await
}

pub async fn verify_code(
    code: web::Json<VerifyCodeReq>,
    session: Session,
) -> Result<HttpResponse, WebError> {
    println!("check_sms_code");

    let expect = session.get::<String>("code")?;
    let expect = expect.unwrap();

    let token: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    let rb = db::init_db();

    // TODO: Check token conflict

    let phone_number = session.get::<String>("phone_number")?;
    let phone_number = phone_number.unwrap();

    db::save(
        &rb.await,
        &db::Member {
            id: None,
            phone_number: Some(phone_number),
            token: Some(token.clone()),
        },
    )
    .await;

    if code.code == expect {
        HttpResponse::Ok().json(VerifyCodeResp { token }).await
    } else {
        utils::get_err_resp().await
    }
}
