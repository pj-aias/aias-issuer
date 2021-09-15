use crate::handler::BasicResponse;
use actix_web::Error as WebError;
use actix_web::HttpResponse;
use regex::Regex;
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

/// validates japanese mobile phone number format.
/// e.g.)
/// "09012345678" -> true
/// "090-1234-5678" -> false
/// "01012345678" -> false
pub fn validate_phone_number(phone: &str) -> bool {
    let phone_pattern = Regex::new("^0[789]0[0-9]{4}[0-9]{4}$").expect("invalid regexp");
    phone_pattern.is_match(phone)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ok_phone_number() {
        let phone = "09012345678";
        assert!(validate_phone_number(phone));
    }

    #[test]
    fn ng_phone_number() {
        let phone = "090-1234-5678";
        assert!(!validate_phone_number(phone));
        let phone = "01012345678";
        assert!(!validate_phone_number(phone));
        let phone = "hogefuga";
        assert!(!validate_phone_number(phone));
    }
}
