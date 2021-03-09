#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate dotenv;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

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
    let changelog_entries = models::Changelog::all_by_map_id("47748".to_string(), &conn);
    //let changelog_entries = models::Changelog::all_by_profile_num("76561198039230536".to_string(), &conn);
    println!("{:#?}", changelog_entries);
}