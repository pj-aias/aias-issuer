use actix_session::CookieSession;
use actix_web::{test, web, App};

use std::env;

use crate::handler;
use crate::handler::{IssueCredReq, SendCodeReq, VerifyCodeReq};

use serde_json;

#[actix_rt::test]
async fn test() {
    let app = App::new()
        .wrap(CookieSession::private(&[0; 32]).secure(true))
        .route("/send_code", web::post().to(handler::send_code))
        .route("/verify_code", web::post().to(handler::verify_code))
        .route(
            "/issue_credential",
            web::post().to(handler::issue_credential),
        );

    let mut app = test::init_service(app).await;

    let phone_number = env::var("SMS_TO").unwrap_or("000-000-0000".to_string());
    let phone_req = SendCodeReq {
        phone_number: phone_number,
    };
    let phone_req = serde_json::to_string(&phone_req).unwrap();

    let req = test::TestRequest::post()
        .uri("/send_code")
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

    let check_sms_req = VerifyCodeReq { code: expect };
    let check_sms_req = serde_json::to_string(&check_sms_req).unwrap();
    let check_sms_req = test::TestRequest::post()
        .uri("/verify_code")
        .cookie(cookie.clone())
        .set_payload(check_sms_req)
        .header("Content-Type", "text/json")
        .to_request();

    let resp = test::call_service(&mut app, check_sms_req).await;

    assert!(resp.status().is_success());

    let token = test::read_body(resp).await;
    let token = String::from_utf8(token.to_vec()).unwrap();

    println!("token: {}", token);

    let issue_cred_req = test::TestRequest::post()
        .uri("/issue_credential")
        .set_payload(token)
        .header("Content-Type", "text/json")
        .to_request();

    let resp = test::call_service(&mut app, issue_cred_req).await;

    assert!(resp.status().is_success());

    let cred = test::read_body(resp).await;
    let cred = String::from_utf8(cred.to_vec()).unwrap();

    println!("cred:{}", cred);
}

#[actix_rt::test]
async fn test_forbidden() {
    let app = App::new()
        .wrap(CookieSession::private(&[0; 32]).secure(true))
        .route(
            "/issue_credential",
            web::post().to(handler::issue_credential),
        );

    let mut app = test::init_service(app).await;

    let req = IssueCredReq {
        token: "hogehoge".to_string(),
    };
    let req = serde_json::to_string(&req).unwrap();

    let req = test::TestRequest::post()
        .uri("/issue_credential")
        .set_payload(req)
        .header("Content-Type", "text/json")
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    let resp = resp.response();

    let status = resp.status();

    assert_eq!(status, 401)
}
