mod config;
mod models;
use crate::models::Status;

use actix_web::{ web, App, HttpResponse, HttpServer, Responder};
use std::io;
use dotenv::dotenv;

// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

async fn status() -> impl Responder {
    HttpResponse::Ok().json(Status {status: "Ok".to_string()})
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let config = crate::config::Config::from_env().unwrap();

    println!("Starting server at http://{}:{}", config.server.host, config.server.port);

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(status))  // .service(hello)
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}