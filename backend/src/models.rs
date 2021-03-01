use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use diesel::sql_types::Timestamp;
use chrono::NaiveDateTime;
//use diesel::deserialize::FromSql;

// http://diesel.rs/guides/schema-in-depth/

use crate::schema::changelog;
use crate::schema::dsl::changelog;

#[derive(Queryable)]
pub struct Changelog {
    pub time_gained: Timestamp,
    pub profile_number: String,
    pub score: i32,
    pub map_id: String,
    pub wr_gain: i32,
    pub has_demo: Option<i32>, // NULLABLE
    pub banned: i32,
    pub youtube_id: Option<String>, // NULLABLE
    pub previous_id: Option<i32>, // NULLABLE
    pub id: i32,
    pub coopid: Option<i32>, // NULLABLE
    pub post_rank: Option<i32>, // NULLABLE
    pub pre_rank: Option<i32>, // NULLABLE
    pub submission: i32,
    pub note: Option<String>, // NULLABLE
    pub category: Option<String>, // NULLABLE
}

#[derive(Insertable)]
#[table_name = "changelog"]
pub struct NewChangelog{
    pub time_gained: NaiveDateTime,
    pub profile_number: String,
    pub score: i32,
    pub map_id: String,
    pub wr_gain: i32,
    pub has_demo: Option<i32>, // NULLABLE
    pub banned: i32,
    pub youtube_id: Option<String>, // NULLABLE
    pub previous_id: Option<i32>, // NULLABLE
    pub coopid: Option<i32>, // NULLABLE
    pub post_rank: Option<i32>, // NULLABLE
    pub pre_rank: Option<i32>, // NULLABLE
    pub submission: i32,
    pub note: Option<String>, // NULLABLE
    pub category: Option<String>, // NULLABLE
}