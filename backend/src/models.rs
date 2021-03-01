use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use diesel::sql_types::Timestamp;
//use chrono::NaiveDateTime;
//use diesel::deserialize::FromSql;

// http://diesel.rs/guides/schema-in-depth/

use crate::schema::changelog;
use crate::schema::dsl::*;

#[derive(Queryable)]
pub struct Changelog {
    pub time_gained: Timestamp,
    pub profile_number: String,
    pub score: i32,
    pub map_id: String,
    pub wr_gain: i32,
    pub has_demo: i32, // NULLABLE
    pub banned: i32,
    pub youtube_id: String, // NULLABLE
    pub previous_id: i32, // NULLABLE
    pub id: i64,
    pub coopid: i64, // NULLABLE
    pub post_rank: i32, // NULLABLE
    pub pre_rank: i32, // NULLABLE
    pub submission: i32,
    pub note: String, // NULLABLE
    pub category: String, // NULLABLE
}

#[derive(Insertable)]
#[table_name = "changelog"]
pub struct NewChangelog{
    pub time_gained: Timestamp,
    pub profile_number: String,
    pub score: i32,
    pub map_id: String,
    pub wr_gain: i32,
    pub has_demo: i32, // NULLABLE
    pub banned: i32,
    pub youtube_id: String, // NULLABLE
    pub previous_id: i32, // NULLABLE
    pub coopid: i64, // NULLABLE
    pub post_rank: i32, // NULLABLE
    pub pre_rank: i32, // NULLABLE
    pub submission: i32,
    pub note: String, // NULLABLE
    pub category: String, // NULLABLE
}