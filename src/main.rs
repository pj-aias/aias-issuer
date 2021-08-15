use actix_session::CookieSession;
use actix_web::{web, App, HttpServer};

mod handler;
use std::io;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(CookieSession::signed(&[0; 32]).secure(true))
            // .data(data.clone())
            .route("/hello", web::get().to(handler::hello))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
