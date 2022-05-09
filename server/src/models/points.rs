use super::changelog::MapScoreDate;
use std::collections::HashMap;

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
