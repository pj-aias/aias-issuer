use crate::handler::BasicResponse;
use actix_web::Error as WebError;
use actix_web::HttpResponse;
use std::env;
use twilio::{Client, OutboundMessage};

pub async fn send_sms(to: &str, body: &str) -> Result<(), ()> {
    let from = env::var("SMS_FROM").expect("Find ACCOUNT_ID environment variable");
    let app_id = env::var("ACCOUNT_ID").expect("Find ACCOUNT_ID environment variable");
    let auth_token = env::var("AUTH_TOKEN").expect("Find AUTH_TOKEN environment variable");

    let client = Client::new(&app_id, &auth_token);
    let msg = OutboundMessage::new(&from, &to, &body);

    match client.send_message(msg).await {
        Ok(_) => return Ok(()),
        Err(e) => {
            println!("{:?}", e);
            return Err(());
        }
    }
}

pub async fn get_err_resp() -> Result<HttpResponse, WebError> {
    HttpResponse::Forbidden().json(BasicResponse {}).await
}