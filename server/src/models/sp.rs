use super::changelog::Changelog;
use chrono::NaiveDateTime;
use sqlx::FromRow;

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
// #[derive(Serialize, Deserialize)]
// pub struct SpPreviews {
//     pub map_id: String,
//     pub scores: Vec<SpPreview>,
// }

/// Wrapper for a player's SP PB history.
#[derive(Serialize, Deserialize)]
pub struct SpPbHistory {
    pub user_name: Option<String>,
    pub avatar: Option<String>,
    pub pb_history: Option<Vec<Changelog>>,
}
/// Wrapper for the sp map data and the rank/score.
#[derive(Serialize)]
pub struct SpRanked {
    pub map_data: SpMap,
    pub rank: i32,
    pub points: f32,
}

/// Banned times for SP
#[derive(Serialize, FromRow)]
pub struct SpBanned {
    pub profile_number: String,
    pub score: i32,
}
