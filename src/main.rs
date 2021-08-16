use actix_session::CookieSession;
use actix_web::{web, App, HttpServer};

use rand::Rng;

mod handler;
mod tests;
mod utils;

use std::io;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let mut key = [0; 32];
    let mut rng = rand::thread_rng();

    for i in 0..32 {
        key[i] = rng.gen();
    }

    HttpServer::new(move || {
        App::new()
            .wrap(CookieSession::private(&key).secure(true))
            // .data(data.clone())
            .route("/hello", web::get().to(handler::hello))
            .route("/prepare_code", web::post().to(handler::prepare_sms_auth))
            .route("/check_code", web::post().to(handler::check_sms_code))
    })
    
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
