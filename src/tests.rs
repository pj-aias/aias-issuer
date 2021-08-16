use actix_session::CookieSession;
use actix_web::{test, web, App};

use std::env;

use crate::handler;
use crate::handler::{CheckSMSAuthReq, PrepareSMSAuthReq};

use serde_json;

#[actix_rt::test]
async fn test_sms_auth() {
    let app = App::new()
        .wrap(CookieSession::private(&[0; 32]).secure(true))
        // .data(data.clone())
        .route("/prepare_code", web::post().to(handler::prepare_sms_auth))
        .route("/check_verify", web::post().to(handler::check_sms_code));

    let mut app = test::init_service(app).await;

    let phone_number = env::var("SMS_TO").unwrap_or("000-000-0000".to_string());
    let phone_req = PrepareSMSAuthReq {
        phone_number: phone_number,
    };
    let phone_req = serde_json::to_string(&phone_req).unwrap();

    let req = test::TestRequest::post()
        .uri("/prepare_code")
        .set_payload(phone_req)
        .header("Content-Type", "text/json")
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    let resp = resp.response();

    let cookie = resp
        .clone()
        .cookies()
        .find(|c| c.name() == "actix-session")
        .expect("failed to get id from response's session");

    assert!(resp.status().is_success());

    let expect = env::var("AIAS_TEST_CODE").expect("Find SECRET environment variable");

    let check_sms_req = CheckSMSAuthReq { code: expect };
    let check_sms_req = serde_json::to_string(&check_sms_req).unwrap();
    let check_sms_req = test::TestRequest::post()
        .uri("/check_verify")
        .cookie(cookie.clone())
        .set_payload(check_sms_req)
        .header("Content-Type", "text/json")
        .to_request();

    let resp = test::call_service(&mut app, check_sms_req).await;

    assert!(resp.status().is_success());

    let bytes = test::read_body(resp).await;
    let bytes = String::from_utf8(bytes.to_vec()).unwrap();

    println!("data: {}", bytes);
}
