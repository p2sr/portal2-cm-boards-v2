use chrono::NaiveDateTime;
use sqlx::FromRow;

/// One-to-one struct for coop_bundled data.
#[derive(Serialize, Deserialize, FromRow)]
pub struct CoopBundled {
    pub id: i64,
    pub p_id1: String,
    pub p_id2: Option<String>,
    pub p1_is_host: Option<bool>,
    pub cl_id1: i64,
    pub cl_id2: Option<i64>,
    pub updated: Option<NaiveDateTime>,
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
    pub submission1: i32,
    pub submission2: i32,
    pub note1: Option<String>,
    pub note2: Option<String>,
    pub category_id: i32,
    pub user_name1: String,
    pub user_name2: Option<String>,
    pub avatar1: Option<String>,
    pub avatar2: Option<String>,
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
    pub map_id: String,
}

/// Wrapper for the coop map data and the rank/score.
#[derive(Serialize)]
pub struct CoopRanked {
    pub map_data: CoopMap,
    pub rank: i32,
    pub points: f32,
}

#[derive(Deserialize, Serialize, Debug, FromRow)]
pub struct CoopTempUser {
    pub cl_id: i64,
    pub profile_number: String,
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

/// Banned times for Coop
#[derive(Serialize, FromRow)]
pub struct CoopBanned {
    pub profile_number1: String,
    pub profile_number2: Option<String>,
    pub score: i32,
}
