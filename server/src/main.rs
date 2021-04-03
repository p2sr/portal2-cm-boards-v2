#![allow(unused_imports)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate dotenv;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use actix_web::{get, body::Body, http::header, web, App, HttpRequest, HttpServer,HttpResponse, Responder, Error, middleware::Logger};
use env_logger::Env;
use dotenv::dotenv;
use diesel::r2d2::{self, ConnectionManager};
use diesel::mysql::MysqlConnection;

mod config;
mod handlers;
mod schema;
mod models;
mod db;
mod structs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // Use config.rs to extract a configuration struct from .env (See documentation about changing .env.example)
    let config = crate::config::Config::from_env().unwrap();
    // Database pool, uses manager to build new database pool, saved in web::Data.
    // Reference Code: https://github.com/actix/examples/blob/master/database_interactions/diesel/src/main.rs
    let manager = ConnectionManager::<MysqlConnection>::new(config.database.database_url);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");

    // Initializes Logger with "default" format:  %a %t "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
    // Remote-IP, Time, First line of request, Response status, Size of response in bytes, Referer, User-Agent, Time to serve
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    println!("Server starting at http://{}:{}/", config.server.host, config.server.port);
      
    // Start our web server, mount and set up routes, data, wrapping, middleware and loggers
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(pool.clone())
            .configure(handlers::init)
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
