#[macro_use]
extern crate rbatis;

use actix_session::CookieSession;
use actix_web::{web, App, HttpServer};

use rand::Rng;

mod db;
mod handler;
mod tests;
mod utils;

use std::io;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let mut rng = rand::thread_rng();
    let key: [u8; 32] = rng.gen();

    let _ = db::init_db();

    HttpServer::new(move || {
        App::new()
            .wrap(CookieSession::private(&key).secure(false))
            .route("/hello", web::get().to(handler::hello))
            .route("/send_code", web::post().to(handler::send_code))
            .route("/verify_code", web::post().to(handler::verify_code))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
