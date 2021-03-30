#![allow(unused_imports)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate dotenv;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

extern crate env_logger;

use actix_web::{get, middleware, body::Body, http::header, web, App, HttpRequest, HttpServer,HttpResponse, Responder, Error};
use dotenv::dotenv;
use diesel::r2d2::{self, ConnectionManager};
use diesel::mysql::MysqlConnection;

mod config;
mod handlers;
mod schema;
mod models;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let config = crate::config::Config::from_env().unwrap();

    // https://github.com/actix/examples/blob/master/database_interactions/diesel/src/main.rs

    let manager = ConnectionManager::<MysqlConnection>::new(config.database.database_url);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");
    env_logger::init();
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(pool.clone())
            .configure(handlers::init)
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}