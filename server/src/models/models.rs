use chrono::NaiveDateTime;
use sqlx::FromRow;
use std::collections::HashMap;

//
// Database
//

/// Empty struct to allow for implementation blocks for admin specific db interactions
pub struct Admin {}

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

/// Wrapper around an optional i32, for use in [actix_web::web::Query]
#[derive(Debug, Serialize, Deserialize)]
pub struct AdminLevel {
    pub admin_level: Option<i32>,
}

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
    pub submission: bool,
    pub note: Option<String>,
    pub category_id: i32,
    pub score_delta: Option<i32>,
    pub verified: Option<bool>,
    pub admin_note: Option<String>,
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
    pub submission: bool,
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

/// One-to-one struct for Category data.
#[derive(Serialize, Deserialize, FromRow)]
pub struct Categories {
    pub id: i32,
    pub name: String,
    pub map_id: String,
    pub rules: String,
}

/// One-to-one struct for chapter data.
#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Chapters {
    pub id: i32,
    pub chapter_name: Option<String>,
    pub is_multiplayer: bool,
    pub game_id: i32,
}

/// Querying for Chapters
#[derive(Serialize, Deserialize, Debug)]
pub struct ChapterQueryParams {
    pub chapter_name: Option<String>,
    pub is_multiplayer: Option<bool>,
    pub game_id: Option<i32>,
}

/// One-to-one struct for coop_bundled data.
#[derive(Serialize, Deserialize, FromRow)]
pub struct CoopBundled {
    pub id: i64,
    pub p_id1: String,
    pub p_id2: Option<String>,
    pub p1_is_host: Option<bool>,
    pub cl_id1: i64,
    pub cl_id2: Option<i64>,
}

/// Insert struct for creating a new `CoopBundled`
#[derive(Serialize, Deserialize, FromRow)]
pub struct CoopBundledInsert {
    pub p_id1: String,
    pub p_id2: Option<String>,
    pub p1_is_host: Option<bool>,
    pub cl_id1: i64,
    pub cl_id2: Option<i64>,
}

/// One-to-one struct for demo data.
#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct Demos {
    pub id: i64,
    pub file_id: String,
    pub partner_name: Option<String>,
    pub parsed_successfully: bool,
    pub sar_version: Option<String>,
    pub cl_id: i64,
}

/// Insert struct for `Demos`, excludes `id`
#[derive(Debug, Default, Serialize, Deserialize, FromRow, Clone)]
pub struct DemoInsert {
    pub file_id: String,
    pub partner_name: Option<String>,
    pub parsed_successfully: bool,
    pub sar_version: Option<String>,
    pub cl_id: i64,
}

/// One-to-one struct for game data.
#[derive(Serialize, Deserialize, FromRow)]
pub struct Games {
    pub id: i32,
    pub game_name: String,
}

/// Query wrapper for game_id
#[derive(Serialize, Deserialize, Debug)]
pub struct GameID {
    pub game_id: Option<i32>,
}

/// One-to-one struct for map data.
#[derive(Serialize, Deserialize, FromRow)]
pub struct Maps {
    pub id: i32,
    pub steam_id: String,
    pub lp_id: String,
    pub name: String,
    pub chapter_id: Option<i32>,
    pub default_cat_id: i32,
    pub is_public: bool,
}

/// One-to-one struct for user data.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Users {
    pub profile_number: String,
    pub board_name: Option<String>,
    pub steam_name: Option<String>,
    pub banned: bool,
    pub registered: i32,
    pub avatar: Option<String>,
    pub twitch: Option<String>,
    pub youtube: Option<String>,
    pub title: Option<String>,
    pub admin: i32,
    pub donation_amount: Option<String>,
    pub discord_id: Option<String>,
}

/// Includes only a `user_name` and `avatar`, does not include the `profile_number`
///
/// Used for when the `profile_number` is included in another portion of the returned values.
#[derive(Debug, FromRow, Deserialize, Serialize, Clone)]
pub struct UsersPage {
    pub user_name: String,
    pub avatar: String,
}

/// Wraps `profile_number`, `user_name` and `avatar` for displaying a user.
#[derive(Debug, FromRow, Deserialize, Serialize, Clone)]
pub struct UsersDisplay {
    pub profile_number: String,
    pub user_name: String,
    pub avatar: String,
}

/// Social media accounts from `Users`
#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Socials {
    pub twitch: Option<String>,
    pub youtube: Option<String>,
    pub discord_id: Option<String>,
}

/// The minimal data we want for SP map pages to lower bandwidth usage.
#[derive(Serialize, FromRow, Debug)]
pub struct SpMap {
    pub timestamp: Option<NaiveDateTime>,
    #[sqlx(rename = "cl_profile_number")]
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
#[derive(Serialize, FromRow, Clone)]
pub struct CoopMap {
    pub timestamp: Option<NaiveDateTime>,
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
pub struct SpRanked {
    pub map_data: SpMap,
    pub rank: i32,
    pub points: f32,
}

/// Wrapper for the coop map data and the rank/score.
#[derive(Serialize)]
pub struct CoopRanked {
    pub map_data: CoopMap,
    pub rank: i32,
    pub points: f32,
}

/// The data for the preview page for all SP Maps
#[derive(Serialize, Deserialize, FromRow)]
pub struct SpPreview {
    #[sqlx(rename = "cl_profile_number")]
    pub profile_number: String,
    pub score: i32,
    pub youtube_id: Option<String>,
    pub category_id: i32,
    pub user_name: String,
    pub map_id: String,
}

/// Wrapper for multiple SpPreviews, prevents repeat data (multiple map_name and map_id copies)
#[derive(Serialize, Deserialize)]
pub struct SpPreviews {
    pub map_id: String,
    pub scores: Vec<SpPreview>,
}

/// Wrapper for a player's SP PB history.
#[derive(Serialize, Deserialize)]
pub struct SpPbHistory {
    pub user_name: Option<String>,
    pub avatar: Option<String>,
    pub pb_history: Option<Vec<Changelog>>,
}

/// The data for the preview page for all Coop Maps
#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct CoopPreview {
    pub profile_number1: String,
    pub profile_number2: Option<String>,
    pub score: i32,
    pub youtube_id1: Option<String>,
    pub youtube_id2: Option<String>,
    pub category_id: i32,
    pub user_name1: String,
    pub user_name2: Option<String>,
}

/// Wrapper for prevciewing the top 7 for all Coop maps.
#[derive(Serialize, Deserialize)]
pub struct CoopPreviews {
    pub map_id: String,
    pub scores: Vec<CoopPreview>,
}

/// Banned times for SP
#[derive(Serialize, FromRow)]
pub struct SpBanned {
    pub profile_number: String,
    pub score: i32,
}

/// Banned times for Coop
#[derive(Serialize, FromRow)]
pub struct CoopBanned {
    pub profile_number1: String,
    pub profile_number2: Option<String>,
    pub score: i32,
}

//
// Helpers
//

/// Values that we return after checking if a score is valid to be added to the database.
#[derive(Default, Debug, Serialize)]
pub struct CalcValues {
    pub previous_id: Option<i64>,
    pub post_rank: Option<i32>,
    pub pre_rank: Option<i32>,
    pub score_delta: Option<i32>,
    pub banned: bool,
}

// Currently a dumbass work around to issues with deserializing an option natively theough the Query
/// Generic wrapper around an Option i32 for [actix_web::web::Query]
#[derive(Debug, Deserialize)]
pub struct OptCatID {
    pub cat_id: Option<i32>,
}

/// Wrapper to send a profile number as a search result
#[derive(Serialize, Deserialize, Debug)]
pub struct ScoreParams {
    pub profile_number: String,
    pub score: i32,
    pub cat_id: Option<i32>,
}

/// Wrapper to allow queries to include `map_id`, `profile_number` and optional `cat_id`.
#[derive(Serialize, Deserialize, Debug)]
pub struct HistoryParams {
    pub profile_number: String,
    pub map_id: String,
    pub cat_id: Option<i32>,
}

/// All the accepted query parameters for the changelog page.
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
}

/// Allows us to accept an optional demo_id or cl_id as a set of query parameters for demo endpoints.
///
/// Intended to be used exclusively (you should either use one or the other, never both or neither).
#[derive(Debug, Clone, Deserialize)]
pub struct DemoOptions {
    pub demo_id: Option<i64>,
    pub cl_id: Option<i64>,
}

/// Used to lookup information on a specific score.
#[derive(Serialize, Deserialize, Debug)]
pub struct ScoreLookup {
    pub profile_number: String,
    pub score: i32,
    pub map_id: String,
    pub cat_id: Option<i32>,
}

// Points

/// Wrapper for us receiving points from the backend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointsReceiveWrapper {
    pub id: Option<i32>,
    pub hm_points: HashMap<String, Points>,
    pub ordered_points: Vec<(String, Points)>,
}

/// Wrapper for writing points, uses a ref to the points to avoid unnecessary allocation, cannot be used to Deserialize.
#[derive(Debug, Clone, Serialize)]
pub struct PointsWriteWrapper<'a> {
    pub id: Option<i32>,
    pub points: &'a Vec<(String, Points)>,
}

/// Wrapper for reading points from a file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointsReadWrapper {
    pub id: Option<i32>,
    pub points: Vec<(String, Points)>,
}

/// Point information for a given player.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Points {
    pub points: f32,
    pub score: i32, // TODO: Need to change the format to support SAR timing
    pub num_scores: i32,
    pub total_rank_sum: i32,
    pub worst: (i32, String),
    pub best: (i32, String),
    pub user_name: Option<String>,
    pub avatar: Option<String>,
}

/// Map ID & Name, score and timestamp for a given score.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MapScoreDate {
    pub map: String,
    pub map_name: String,
    pub score: i32,
    pub timestamp: Option<NaiveDateTime>,
}

/// Oldest and newest `MapScoreDate` for a profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileData {
    pub oldest_sp: MapScoreDate,
    pub newest_sp: MapScoreDate,
    pub oldest_coop: MapScoreDate,
    pub newest_coop: MapScoreDate,
}

/// Wrapper for a profile page, includes the ID associated with the points and the poits themselves.
#[derive(Debug, Clone, Serialize)]
pub struct PointsProfileWrapper {
    pub id: i32,
    pub points: Points,
}

/// Profile Page that includes a Vec of PointsProfileWrappers, ProfileData and a hasmap of map_ids to current ranks.
#[derive(Debug, Clone, Serialize)]
pub struct ProfilePage {
    pub points: Vec<PointsProfileWrapper>,
    pub data: ProfileData,
    pub ranks: HashMap<String, i32>,
}
