#[macro_use]
extern crate rbatis;

use actix_session::CookieSession;
use actix_web::{web, App, HttpServer};
use std::sync::Arc;
use std::sync::Mutex;

use rand::Rng;

mod db;
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

    let rb = db::init_db();
    let data = Arc::new(Mutex::new(rb));

    HttpServer::new(move || {
        App::new()
            .wrap(CookieSession::private(&key).secure(true))
            .data(data.clone())
            .route("/hello", web::get().to(handler::hello))
            .route("/send_code", web::post().to(handler::send_code))
            .route("/verify_code", web::post().to(handler::verify_code))
            .route(
                "/issue_credential",
                web::post().to(handler::issue_credential),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
