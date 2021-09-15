use crate::db::Member;
use actix_session::Session;
use actix_web::Error as WebError;
use actix_web::{web, HttpResponse, Responder};
use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::rsa::Rsa;
use openssl::sign::Signer;

use crate::utils;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::env;

use crate::db;
use crate::rbatis::crud::CRUD;

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
    pub pubkey: String,
}

#[derive(Deserialize, Serialize)]
pub struct VerifyCodeResp {
    pub cert: String,
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

    let is_debugging = env::var("AIAS_DEBUG").unwrap_or("true".to_string());
    let mut code: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    let phone_number = &phone_number.phone_number;
    if !utils::validate_phone_number(phone_number) {
        return utils::get_err_resp(format!("invalid phone number: {}", phone_number)).await;
    }

    let body = format!("code {}", code);

    if is_debugging == "true" {
        code = "0000".to_string();
        env::set_var("AIAS_TEST_CODE", code.clone());
    } else {
        match utils::send_sms(phone_number, &body).await {
            Ok(_) => {}
            Err(e) => return utils::get_err_resp(e).await,
        }
    }

    session.set::<String>("phone_number", phone_number.to_string())?;
    session.set::<String>("code", code)?;

    HttpResponse::Ok().json(BasicResponse {}).await
}

pub async fn verify_code(
    req: web::Json<VerifyCodeReq>,
    session: Session,
) -> Result<HttpResponse, WebError> {
    println!("check_sms_code");

    let privkey = env::var("AIAS_ISSUER_PRIVKEY").expect("pem is not found");

    let expect = session.get::<String>("code")?.unwrap();
    // let expect = expect.unwrap();

    let code = &req.code;
    let user_pubkey = &req.pubkey;

    println!("code: {}, {}", code, expect);
    if code != &expect {
        return utils::get_err_resp(format!("bad code: {} (expected {})", code, expect)).await;
    };

    let rb = db::init_db().await;

    let phone_number = session.get::<String>("phone_number")?;
    let phone_number = phone_number.unwrap();

    match rb
        .fetch_by_column::<Member, String>("phone_number", &phone_number)
        .await
    {
        Ok(_) => return utils::get_err_resp(format!("phone number found: {}", phone_number)).await,
        Err(_) => {}
    };

    let privkey = Rsa::private_key_from_pem(&privkey.as_bytes()).expect("private key is not valid");
    let pubkey = PKey::from_rsa(privkey).expect("key generation error");

    let mut signer = Signer::new(MessageDigest::sha256(), &pubkey).expect("sign error");
    signer.update(user_pubkey.as_bytes()).expect("sign error");

    let cert = signer.sign_to_vec().expect("sign error");
    let cert = base64::encode(cert);

    db::save(
        &rb,
        &db::Member {
            id: None,
            phone_number: Some(phone_number),
            cert: Some(cert.clone()),
        },
    )
    .await;

    HttpResponse::Ok()
        .json(VerifyCodeResp {
            cert: cert.to_string(),
        })
        .await
}
