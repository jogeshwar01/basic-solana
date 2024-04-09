mod config;
mod models;
mod handler;
mod db;
use crate::handler::status;

use actix_web::{ web::{self, Data}, App, HttpServer};
use std::io;
use dotenv::dotenv;
use tokio_postgres::NoTls;
use handler::get_todos;

#[actix_web::main]
async fn main() -> io::Result<()> {
    // for debug logs
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    
    dotenv().ok();
    let config = crate::config::Config::from_env().unwrap();

    let pool = config.pg.create_pool(NoTls).unwrap();

    println!("Starting server at http://{}:{}", config.server.host, config.server.port);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .route("/", web::get().to(status))  // .service(hello)
            .route("/todos{_:/?}",web::get().to(get_todos))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}