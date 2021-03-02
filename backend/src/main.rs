#[allow(dead_code)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate dotenv;

use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

mod schema;
mod models;

fn main() {
    println!("Hello, world!");
}
