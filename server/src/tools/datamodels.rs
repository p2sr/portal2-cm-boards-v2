#![allow(unused)]
#![allow(clippy::all)]

use std::collections::HashMap;
use actix_web::{HttpResponse, Error};
use sqlx::{FromRow, Postgres, types::Type};
use sqlx::postgres::PgValueRef;
use sqlx::decode::Decode;
use sqlx::postgres::types::PgRecordDecoder;
use chrono::NaiveDateTime;

// rustc bug: https://github.com/rust-lang/rust/issues/82219
// impl<'r> Decode<'r, Postgres> for Changelog
    // where
    //     // i64: Decode<'r, Postgres>,
    //     // i64: Type<Postgres>,
    //     Option<NaiveDateTime>: Decode<'r, Postgres>,
    //     Option<NaiveDateTime>: Type<Postgres>,
    //     // String: Decode<'r, Postgres>,
    //     // String: Type<Postgres>,
    //     // i32: Decode<'r, Postgres>,
    //     // i32: Type<Postgres>,
    //     Option<String>: Decode<'r, Postgres>,
    //     Option<String>: Type<Postgres>,
    //     // bool: Decode<'r, Postgres>,
    //     // bool: Type<Postgres>,
    //     Option<i64>: Decode<'r, Postgres>,
    //     Option<i64>: Type<Postgres>,
    //     Option<i32>: Decode<'r, Postgres>,
    //     Option<i32>: Type<Postgres>,
    //     Option<bool>: Decode<'r, Postgres>,
    //     Option<bool>: Type<Postgres>,
    // {
    //     fn decode(value: PgValueRef<'r>) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
    //         let mut decoder = PgRecordDecoder::new(value)?;
    //         //let id = decoder.try_decode::<i64>()?;
    //         let timestamp = decoder.try_decode::<Option<NaiveDateTime>>()?;
    //         //let profile_number = decoder.try_decode::<String>()?;
    //         //let score = decoder.try_decode::<i32>()?;
    //         //let map_id = decoder.try_decode::<String>()?;
    //         let demo_id = decoder.try_decode::<Option<i64>>()?;
    //         //let banned = decoder.try_decode::<bool>()?;
    //         let youtube_id = decoder.try_decode::<Option<String>>()?;
    //         let previous_id = decoder.try_decode::<Option<i64>>()?;
    //         let coop_id = decoder.try_decode::<Option<i64>>()?;
    //         let post_rank = decoder.try_decode::<Option<i32>>()?;
    //         let pre_rank = decoder.try_decode::<Option<i32>>()?;
    //         //let submission = decoder.try_decode::<bool>()?;
    //         let note = decoder.try_decode::<Option<String>>()?;
    //         //let category_id = decoder.try_decode::<i32>()?;
    //         let score_delta = decoder.try_decode::<Option<i32>>()?;
    //         let verified = decoder.try_decode::<Option<bool>>()?;
    //         let admin_note = decoder.try_decode::<Option<String>>()?;
    //         Ok(Changelog{
    //             // id,
    //             timestamp,
    //             //profile_number,
    //             // score,
    //             //map_id,
    //             demo_id,
    //             // banned,
    //             youtube_id,
    //             previous_id,
    //             coop_id,
    //             post_rank,
    //             pre_rank,
    //             // submission,
    //             note,
    //             // category_id,
    //             score_delta,
    //             verified,
    //             admin_note,
    //         })
    //     }
    // }


    /// One-to-one struct for changelog data.


#[derive(Serialize, Deserialize, FromRow)]
pub struct Changelog{
    pub id: i64,
    pub timestamp: Option<NaiveDateTime>,
    pub profile_number: String,
    pub score: i32,
    pub map_id: String,
    pub demo_id: Option<i64>,
    pub banned: bool,
    pub youtube_id: Option<String>,
    pub previous_id: Option<i64>,
    pub coop_id: Option<i64>,
    pub post_rank: Option<i32>,
    pub pre_rank: Option<i32>,
    pub submission: bool,
    pub note: Option<String>,
    pub category_id: i32,
    pub score_delta: Option<i32>,
    pub verified: Option<bool>,
    pub admin_note: Option<String>,
}
/// All changelog data except for the ID, for table insertion.
#[derive(Serialize)]
pub struct ChangelogInsert{
    pub timestamp: Option<NaiveDateTime>,
    pub profile_number: String,
    pub score: i32,
    pub map_id: String,
    pub demo_id: Option<i64>,
    pub banned: bool,
    pub youtube_id: Option<String>,
    pub previous_id: Option<i64>,
    pub coop_id: Option<i64>,
    pub post_rank: Option<i32>,
    pub pre_rank: Option<i32>,
    pub submission: bool,
    pub note: Option<String>,
    pub category_id: i32,
    pub score_delta: Option<i32>,
    pub verified: Option<bool>,
    pub admin_note: Option<String>,
}

/// One-to-one struct for Category data.
#[derive(Serialize, Deserialize, FromRow)]
pub struct Categories{
    pub id: i32,
    pub name: String,
    pub map_id: String, 
    pub rules: String,
}

/// One-to-one struct for chapter data.
#[derive(Serialize, Deserialize, FromRow)]
pub struct Chapters{
    pub id: i32,
    pub chapter_name: Option<String>,
    pub is_multiplayer: bool,
    pub game_id: i32,
}

/// One-to-one struct for coop_bundled data.
#[derive(Serialize, Deserialize, FromRow)]
pub struct CoopBundled{
    pub id: i64,
    pub p_id1: String,
    pub p_id2: Option<String>,
    pub p1_is_host: Option<bool>,
    pub cl_id1: i64,
    pub cl_id2: Option<i64>,
}

/// One-to-one struct for demo data.
#[derive(Serialize, Deserialize, FromRow)]
pub struct Demos{
    pub id: i64,
    pub drive_url: String,
    pub partner_name: Option<String>,
    pub parsed_successfully: bool,
    pub sar_version: Option<String>,
    pub cl_id: i64,
}

/// One-to-one struct for game data.
#[derive(Serialize, Deserialize, FromRow)]
pub struct Games{
    pub id: i32,
    pub game_name: String,
}

/// One-to-one struct for map data.
#[derive(Serialize, Deserialize, FromRow)]
pub struct Maps{
    pub id: i32,
    pub steam_id: String,
    pub lp_id: String,
    pub name: String,
    pub chapter_id: Option<i32>,
    pub is_public: bool,
}

/// One-to-one struct for user data.
#[derive(Serialize, Deserialize, FromRow)]
pub struct Users{
    pub profile_number: String,
    pub board_name: Option<String>,
    pub steam_name: Option<String>,
    pub banned: bool,
    pub registred: bool,
    pub avatar: Option<String>,
    pub twitch: Option<String>,
    pub youtube: Option<String>,
    pub title: Option<String>,
    pub admin: i32,
    pub donation_amount: Option<String>,
    pub discord_id: Option<String>,
}
/// The minimal data we want for SP map pages to lower bandwidth usage.
#[derive(Serialize, FromRow)]
pub struct SpMap{
    pub timestamp: Option<NaiveDateTime>,
    pub profile_number: String,
    pub score: i32,
    pub demo_id: Option<i64>,
    pub youtube_id: Option<String>,
    pub submission: bool,
    pub note: Option<String>,
    pub category_id: i32, 
    pub user_name: Option<String>,
    pub avatar: Option<String>,
}

/// The minimal data we want for Coop map pages to lower bandwitch usage.
#[derive(Serialize, FromRow)]
pub struct CoopMap{
    pub time_gained: Option<NaiveDateTime>,
    pub profile_number1: String,
    pub profile_number2: String,
    pub score: i32,
    pub p1_is_host: Option<bool>,
    pub demo_id1: Option<i64>,
    pub demo_id2: Option<i64>,
    pub youtube_id1: Option<String>,
    pub youtube_id2: Option<String>,
    pub submission1: bool,
    pub submission2: bool,
    pub note1: Option<String>,
    pub note2: Option<String>,
    pub category_id: i32,
    pub user_name1: String,
    pub user_name2: Option<String>,
    pub avatar1: Option<String>,
    pub avatar2: Option<String>,
}

/// Wrapper for the sp map data and the rank/score.
#[derive(Serialize)]
pub struct SpRanked{
    pub map_data: SpMap,
    pub rank: i32,
    pub points: f32,
}

/// Wrapper for the coop map data and the rank/score.
#[derive(Serialize)]
pub struct CoopRanked{
    pub map_data1: CoopMap,
    pub map_data2: CoopMap,
    pub timestamp: Option<NaiveDateTime>,
    pub category_id: i32,
    pub score: i32,
    pub rank: i32,
    pub points: f32,
}

/// The data for the preview page for all SP Maps
#[derive(Serialize, Deserialize, FromRow)]
pub struct SpPreview{
    pub profile_number: String,
    pub score: i32,
    pub youtube_id: Option<String>,
    pub category_id: i32, 
    pub user_name: String,
    pub map_id: String
}

/// Wrapper for multiple SpPreviews, prevents repeat data (multiple map_name and map_id copies)
#[derive(Serialize, Deserialize)]
pub struct SpPreviews{
    pub map_id: String,
    pub map_data: Vec<SpPreview>,
}

/// The data for the preview page for all Coop Maps
#[derive(Serialize, FromRow)]
pub struct CoopPreview{
    pub profile_number1: String,
    pub profile_number2: Option<String>,
    pub score: i32,
    pub youtube_id1: Option<String>,
    pub youtube_id2: Option<String>,
    pub category_id: i32, 
    pub user_name1: String,
    pub user_name2: Option<String>,
}

/// Wrapper for prevciewing the top 7 for all Coop maps=.
#[derive(Serialize)]
pub struct CoopPreviews{
    pub map_name: String,
    pub map_id: String,
    pub scores: Vec<CoopPreview>,
}

/// Changelog Wrapper that contains additional information on the changelog page.
// #[derive(Serialize, FromRow)]
// pub struct ChangelogPage{
//     pub cl: Changelog,
//     pub map_name: String,
//     pub user_name: String,
//     pub avatar: String,
// }
// TODO: rustc issues.
#[derive(Serialize, FromRow)]
pub struct ChangelogPage{
    pub id: i64,
    pub timestamp: Option<NaiveDateTime>,
    pub profile_number: String,
    pub score: i32,
    pub map_id: String,
    pub demo_id: Option<i64>,
    pub banned: bool,
    pub youtube_id: Option<String>,
    pub previous_id: Option<i64>,
    pub coop_id: Option<i64>,
    pub post_rank: Option<i32>,
    pub pre_rank: Option<i32>,
    pub submission: bool,
    pub note: Option<String>,
    pub category_id: i32,
    pub score_delta: Option<i32>,
    pub verified: Option<bool>,
    pub admin_note: Option<String>,
    pub map_name: String,
    pub user_name: String,
    pub avatar: String,
}

/// All the accepted query parameters for the changelog page.
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

/// Wrapper to send a profile number as a search result
#[derive(Deserialize, Debug)]
pub struct UserParams{
    pub profilenumber: String,
}

/// Wrapper to send a profile number as a search result
#[derive(Serialize, Deserialize, Debug)]
pub struct ScoreParams{
    pub profilenumber: String,
    pub score: i32,
}

/// Banned times for SP
#[derive(Serialize, FromRow)]
pub struct SpBanned{
    pub profilenumber: String,
    pub score: i32,
}

/// Banned times for Coop
#[derive(Serialize, FromRow)]
pub struct CoopBanned{
    pub profilenumber1: String,
    pub profilenumber2: String,
    pub score: i32,
}

/// Wrapper for a player's SP PB history.
#[derive(Serialize, Deserialize)]
pub struct SpPbHistory{
    pub user_name: String,
    pub avatar: Option<String>,
    pub pb_history: Option<Vec<Changelog>>,
}