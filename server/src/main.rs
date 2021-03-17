#![allow(unused_imports)]

mod config;
mod handlers;

#[macro_use]
extern crate serde_json;

use actix_web::{get, body::Body, http::header, web, App, HttpRequest, HttpServer,HttpResponse, Responder, Error};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let config = crate::config::Config::from_env().unwrap();

    HttpServer::new(|| {
        App::new()
            .configure(handlers::init)
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}