use actix_session::CookieSession;
use actix_web::{test, web, App};

use std::env;

use crate::handler;
use crate::handler::PrepareSMSAuthReq;

use serde_json;

#[actix_rt::test]
async fn test_sms_auth() {
    let app = App::new()
        .wrap(CookieSession::private(&[0; 32]).secure(true))
        // .data(data.clone())
        .route("/hello", web::get().to(handler::hello))
        .route(
            "/prepare_sms_auth",
            web::get().to(handler::prepare_sms_auth),
        );

    let mut app = test::init_service(app).await;

    let phone_number = env::var("SMS_TO").unwrap_or("000-000-0000".to_string());
    let phone_req = PrepareSMSAuthReq {
        phone_number: phone_number,
    };
    let phone_req = serde_json::to_string(&phone_req).unwrap();

    let req = test::TestRequest::post()
        .uri("/prepare_sms_auth")
        .set_payload(phone_req)
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    let bytes = test::read_body(resp).await;

    println!("{:?}", bytes);

    // let cookie = resp
    //     .cookies()
    //     .find(|c| c.name() == "actix-session")
    //     .expect("failed to get id from response's session");
}
