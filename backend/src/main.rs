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
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set the DATABASE_URL in ../.env");
    let conn = MysqlConnection::establish(&database_url).unwrap();
}
