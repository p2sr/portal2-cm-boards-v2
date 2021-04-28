#![allow(unused)]
#![allow(clippy::all)]

use std::collections::HashMap;
use actix_web::{HttpResponse, Error};

use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use chrono::NaiveDateTime;

use crate::tools::schema::{changelog, chapters, coopbundled, maps, scores, usersnew};
use crate::tools::schema::changelog::dsl::changelog as all_changelogs;
use crate::tools::schema::usersnew::dsl::usersnew as all_users;
use crate::tools::schema::maps::dsl::maps as all_maps;
use crate::tools::schema::coopbundled::dsl::coopbundled as all_coops;
use crate::db::DbPool;

/// Structs are generated off the database (using deisel_ext) and modified to be used to store query data.
#[derive(Serialize, Queryable, Debug, Clone, Identifiable)]
#[table_name = "changelog"]
pub struct Changelog {
    pub time_gained: Option<NaiveDateTime>,
    pub profile_number: String,
    pub score: i32,
    pub map_id: String,
    pub wr_gain: i32,
    pub has_demo: Option<i32>,
    pub banned: i32,
    pub youtube_id: Option<String>,
    pub previous_id: Option<i32>,
    pub id: i32,
    pub coopid: Option<i32>,
    pub post_rank: Option<i32>,
    pub pre_rank: Option<i32>,
    pub submission: i32,
    pub note: Option<String>,
    pub category: Option<String>,
}

#[derive(Serialize, Queryable, Debug, Clone, Identifiable)]
#[table_name = "chapters"]
pub struct Chapter {
    pub id: u32,
    pub chapter_name: Option<String>,
    pub is_multiplayer: i32,
}

#[derive(Serialize, Queryable, Debug, Clone, Identifiable)]
#[table_name = "coopbundled"]
pub struct Coopbundled {
    pub time_gained: Option<NaiveDateTime>,
    pub profile_number1: String,
    pub profile_number2: String,
    pub score: i32,
    pub map_id: String,
    pub wr_gain: i32,
    pub is_blue: Option<i32>,
    pub has_demo1: Option<i32>,
    pub has_demo2: Option<i32>,
    pub banned: i32,
    pub youtube_id1: Option<String>,
    pub youtube_id2: Option<String>,
    pub previous_id1: Option<i32>,
    pub previous_id2: Option<i32>,
    pub changelogid1: i32,
    pub changelogid2: i32,
    pub id: i32,
    pub post_rank1: Option<i32>,
    pub post_rank2: Option<i32>,
    pub pre_rank1: Option<i32>,
    pub pre_rank2: Option<i32>,
    pub submission1: i32,
    pub submission2: i32,
    pub note1: Option<String>,
    pub note2: Option<String>,
    pub category: Option<String>,
}

#[derive(Serialize, Queryable, Debug, Clone, Identifiable)]
#[table_name = "maps"]
pub struct Map {
    pub id: i32,
    pub steam_id: String,
    pub lp_id: String,
    pub name: Option<String>,
    pub type_: String,
    pub chapter_id: Option<u32>,
    pub is_coop: i32,
    pub is_public: i32,
}

#[derive(Serialize, Queryable, Debug, Clone, Identifiable)]
#[primary_key(changelog_id)]
#[table_name = "scores"]
pub struct Score {
    pub profile_number: String,
    pub map_id: String,
    pub changelog_id: i32,
}

#[derive(Serialize, Queryable, Debug, Clone, Identifiable)]
#[primary_key(profile_number)]
#[table_name = "usersnew"]
pub struct Usersnew {
    pub profile_number: String,
    pub boardname: Option<String>,
    pub steamname: Option<String>,
    pub banned: i32,
    pub registered: i32,
    pub avatar: Option<String>,
    pub twitch: Option<String>,
    pub youtube: Option<String>,
    pub title: Option<String>,
    pub admin: i32,
    pub donation_amount: Option<String>,
}


/// This struct handles the minimal information we want for SP map pages. We want to limit the amount of data we need to transfer.
#[derive(Queryable, Serialize, Debug, Clone)]
pub struct SPMap{
    pub time_gained: Option<NaiveDateTime>,
    pub profile_number: String,
    pub score: i32,
    pub has_demo: Option<i32>,
    pub youtube_id: Option<String>,
    pub submission: i32,
    pub note: Option<String>,
    pub category: Option<String>, 
    pub boardname: Option<String>,
    pub steamname: Option<String>,
    pub avatar: Option<String>,
}

/// This struct is a work-around for the issues with aliased queries in diesel, ideally this would be scrapped for an aliased join on usersnew
/// so we could grab both sets of usersnew information in one query
/// TODO: Potentially work boardname and steamname into one field? (Check if boardname exists, if it doesn, keep it, if not, replace it with steamname)
#[derive(Queryable, Serialize, Debug, Clone)]
pub struct CoopMapPrelude{
    pub time_gained: Option<NaiveDateTime>,
    pub profile_number1: String,
    pub profile_number2: String,
    pub score: i32,
    pub is_blue: Option<i32>,
    pub has_demo1: Option<i32>,
    pub has_demo2: Option<i32>,
    pub youtube_id1: Option<String>,
    pub youtube_id2: Option<String>,
    pub submission1: i32,
    pub submission2: i32,
    pub note1: Option<String>,
    pub note2: Option<String>,
    pub category: Option<String>,
    pub boardname: Option<String>,
    pub steamname: Option<String>,
    pub avatar: Option<String>,
}

/// This is the big brother struct for the prelude to handle all of the data. The overhead on copy all the data is relatively small, 
/// but ideally we would only need this and not the prelude.
#[derive(Queryable, Serialize, Debug, Clone)]
pub struct CoopMap{
    pub time_gained: Option<NaiveDateTime>,
    pub profile_number1: String,
    pub profile_number2: String,
    pub score: i32,
    pub is_blue: Option<i32>,
    pub has_demo1: Option<i32>,
    pub has_demo2: Option<i32>,
    pub youtube_id1: Option<String>,
    pub youtube_id2: Option<String>,
    pub submission1: i32,
    pub submission2: i32,
    pub note1: Option<String>,
    pub note2: Option<String>,
    pub category: Option<String>,
    pub boardname1: Option<String>,
    pub steamname1: Option<String>,
    pub avatar1: Option<String>,
    pub boardname2: Option<String>,
    pub steamname2: Option<String>,
    pub avatar2: Option<String>,
}

/// Grabs just the essential user information to aid in filling in the CoopMapPrelude
// TODO: Potentially work boardname and steamname into one field? (Check if boardname exists, if it doesn, keep it, if not, replace it with steamname)
#[derive(Queryable, Serialize, Debug, Clone)]
pub struct UserMap{
    pub boardname: Option<String>,
    pub steamname: Option<String>,
    pub avatar: Option<String>,
}

/// SpPreview and SpPreviews generate the time information displayed on the /sp route. 
// These two work together to grab all that data from the database (only what's necessary)
#[derive(Queryable, Serialize, Debug, Clone)]
pub struct SpPreview{
    pub map_id: String,
    pub profile_number: String,
    pub score: i32,
    pub youtube_id: Option<String>,
    pub category: Option<String>, 
    pub boardname: Option<String>,
    pub steamname: Option<String>,
}

/// Preview the top 7 for all SP maps (any%)
#[derive(Queryable, Serialize, Debug, Clone)]
pub struct SpPreviews{
    pub map_name: Option<String>,
    pub scores: Vec<SpPreview>,
}

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct CoopPreviewPrelude{
    pub map_id: String,
    pub profile_number1: String,
    pub profile_number2: String,
    pub score: i32,
    pub youtube_id1: Option<String>,
    pub youtube_id2: Option<String>,
    pub category: Option<String>, 
    pub boardname: Option<String>,
    pub steamname: Option<String>,
}

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct CoopPreview{
    pub map_id: String,
    pub profile_number1: String,
    pub profile_number2: String,
    pub score: i32,
    pub youtube_id1: Option<String>,
    pub youtube_id2: Option<String>,
    pub category: Option<String>, 
    pub boardname1: Option<String>,
    pub steamname1: Option<String>,
    pub boardname2: Option<String>,
    pub steamname2: Option<String>,
}


#[derive(Queryable, Serialize, Debug, Clone)]
pub struct CoopPreviews{
    pub map_name: Option<String>,
    pub scores: Vec<CoopPreview>,
}

#[derive(Serialize, Queryable, Debug, Clone)]
pub struct ChangelogPage{
    pub time_gained: Option<NaiveDateTime>,
    pub profile_number: String,
    pub score: i32,
    pub map_id: String,
    pub wr_gain: i32,
    pub has_demo: Option<i32>,
    pub youtube_id: Option<String>,
    pub previous_id: Option<i32>,
    pub id: i32,
    pub coopid: Option<i32>,
    pub post_rank: Option<i32>,
    pub pre_rank: Option<i32>,
    pub submission: i32,
    pub note: Option<String>,
    pub category: Option<String>,
    pub name: Option<String>,
    pub boardname: Option<String>,
    pub steamname: Option<String>,
    pub avatar: Option<String>,
}

//// From original handlers.rs

// Wrapper for the sp map data and the rank/score
#[derive(Serialize)]
pub struct SPRanked{
    pub map_data: SPMap,
    pub rank: i32,
    pub score: f32,
}

// Wrapper for the coop map data and the rank/score
#[derive(Serialize)]
pub struct CoopRanked{
    pub map_data: CoopMap,
    pub rank: i32,
    pub score: f32,
}

/// All the accepted query parameters. Every parameter is optional except for limit, which controls the # of requests returned.
#[derive(Deserialize, Debug)]
pub struct ChangelogQueryParams{
    pub limit: i32,
    pub nickname: Option<String>,
    pub profilenumber: Option<String>,
    pub chamber: Option<String>,
    pub sp: Option<i32>,
    pub coop: Option<i32>,
    pub wrgain: Option<i32>,
    pub hasdemo: Option<i32>,
    pub yt: Option<i32>,
}
