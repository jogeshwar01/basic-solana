mod models;
use crate::models::Status;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::io;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn status() -> impl Responder {
    HttpResponse::Ok().json(Status {status: "Ok".to_string()})
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .route("/status", web::get().to(status))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}