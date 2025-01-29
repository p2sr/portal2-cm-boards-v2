use chrono::NaiveDateTime;
use chrono::NaiveDate;
use sqlx::FromRow;

use super::users::UsersDisplayCount;

/// One-to-one struct for changelog data.
#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct Changelog {
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
    pub submission: i32,
    pub note: Option<String>,
    pub category_id: i32,
    pub score_delta: Option<i32>,
    pub verified: Option<bool>,
    pub admin_note: Option<String>,
    pub updated: Option<NaiveDateTime>,
}

/// One-to-one struct for evidence_requirements
#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct EvidenceRequirements {
    pub id: i32,
    pub rank: i32,
    pub demo: Option<bool>,
    pub video: Option<bool>,
    pub active: Option<bool>,
    pub timestamp: Option<NaiveDateTime>,
    pub closed_timestamp: Option<NaiveDateTime>,
}

/// All changelog data except for the ID, for table insertion.
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct ChangelogInsert {
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
    pub submission: i32,
    pub note: Option<String>,
    pub category_id: i32,
    pub score_delta: Option<i32>,
    pub verified: Option<bool>,
    pub admin_note: Option<String>,
}

/// Indlues additional information from joins that includes details like map name, username and profile image.
#[derive(Serialize, FromRow, Debug)]
pub struct ChangelogPage {
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
    pub submission: i32,
    pub note: Option<String>,
    pub category_id: i32,
    pub score_delta: Option<i32>,
    pub verified: Option<bool>,
    pub admin_note: Option<String>,
    pub map_name: String,
    pub user_name: String,
    pub avatar: String,
    pub blue_name: Option<String>,
    pub orange_name: Option<String>,
    pub blue_avatar: Option<String>,
    pub orange_avatar: Option<String>,
}

/// Indlues additional information from joins that includes details like map name, username and profile image.
#[derive(Serialize, FromRow, Debug)]
pub struct Graph {
    pub date: Option<NaiveDate>,
    pub count: i64,
}

// Helpers

/// Details on a user's banned/unverified runs for the admin display page
#[derive(Debug, FromRow, Deserialize, Serialize, Clone)]
pub struct BannedTimeDetails {
    pub profile_number: String,
    pub user_name: String,
    pub avatar: String,
    pub total_runs: i64,
    pub banned_runs: Option<i64>,
    pub non_verified_runs: Option<i64>,
}

/// Values that we return after checking if a score is valid to be added to the database.
#[derive(Default, Debug, Serialize)]
pub struct CalcValues {
    pub previous_id: Option<i64>,
    pub post_rank: Option<i32>,
    pub pre_rank: Option<i32>,
    pub score_delta: Option<i32>,
    pub banned: bool,
}

/// Wrapper to send a profile number as a search result
#[derive(Serialize, Deserialize, Debug)]
pub struct ScoreParams {
    pub profile_number: String,
    pub score: i32,
    pub cat_id: Option<i32>,
    pub game_id: Option<i32>,
}

/// Wrapper to allow queries to include `map_id`, `profile_number` and optional `cat_id`.
#[derive(Serialize, Deserialize, Debug)]
pub struct HistoryParams {
    pub profile_number: String,
    pub map_id: String,
    pub cat_id: Option<i32>,
    pub game_id: Option<i32>,
}

/// All the accepted query parameters for the SubmissionChangelog page.
#[derive(Deserialize, Debug)]
pub struct ChangelogQueryParams {
    pub limit: Option<u32>,
    pub nick_name: Option<String>,
    pub profile_number: Option<String>,
    pub chamber: Option<String>,
    pub sp: Option<bool>,
    pub coop: Option<bool>,
    pub wr_gain: Option<bool>,
    pub has_demo: Option<bool>,
    pub yt: Option<bool>,
    pub first: Option<i64>,
    pub last: Option<i64>,
}

/// Query parameters for searching for a given
#[derive(Deserialize, Debug)]
pub struct ChangelogSearchQuery {
    pub profile_number: String,
    pub map_id: String,
    pub score: i32,
    pub category_id: Option<i32>,
    pub game_id: Option<i32>,
}

/// Fields for a submission to the changelog
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubmissionChangelog {
    pub timestamp: String,
    pub profile_number: String,
    pub score: i32,
    pub map_id: String,
    pub youtube_id: Option<String>,
    pub note: Option<String>,
    pub category_id: Option<i32>,
    pub game_id: Option<i32>,
}
/// Used to lookup information on a specific score.
#[derive(Serialize, Deserialize, Debug)]
pub struct ScoreLookup {
    pub profile_number: String,
    pub score: i32,
    pub map_id: String,
    pub cat_id: Option<i32>,
    pub game_id: Option<i32>,
}

/// Map ID & Name, score and timestamp for a given score.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MapScoreDate {
    pub map: String,
    pub map_name: String,
    pub score: i32,
    pub timestamp: Option<NaiveDateTime>,
}

/// Used to count the number of scores per-user.
#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct NumScores {
    pub count: i64,
    pub profile_number: String,
    pub user_name: String,
    pub avatar: String,
}

/// Used to represent users and their score deltas on a given map.
#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct ScoreDeltaComparison {
    pub profile_number: String,
    pub user_name: String,
    pub avatar: String,
    pub score_delta: i32,
    pub map_id: String,
    pub map_name: String,
}

/// Representation of the number of World Records per Map for a given map.
#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct NumUpdatePerMap {
    pub map_id: String,
    pub map_name: String,
    pub count: i64,
}
/// Struct for the "Recap", taken from NeKz's recap bot on the Discord server.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Recap {
    pub num_wrs: Vec<UsersDisplayCount>,
    pub num_demos: Vec<UsersDisplayCount>,
    pub top_wr_diff: Vec<ScoreDeltaComparison>,
    pub most_updates: Vec<UsersDisplayCount>,
    pub top_videos: Vec<UsersDisplayCount>,
    pub top_score_by_map: Vec<NumUpdatePerMap>,
}
