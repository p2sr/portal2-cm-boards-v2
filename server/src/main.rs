#![allow(unused_imports)]

mod config;
mod handlers;

#[macro_use]
extern crate serde_json;

use actix_web::{get, body::Body, http::header, web, App, HttpRequest, HttpServer,HttpResponse, Responder, Error, middleware::Logger};
use env_logger::Env;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let config = crate::config::Config::from_env().unwrap();

    // Initializes Logger with "default" format:  %a %t "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
    // Remote-IP, Time, First line of request, Response status, Size of response in bytes, Referer, User-Agent, Time to serve
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    println!("Server starting at http://{}:{}/", config.server.host, config.server.port);

    HttpServer::new(|| {
        App::new()
        	.wrap(Logger::default())
            .configure(handlers::init)
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}