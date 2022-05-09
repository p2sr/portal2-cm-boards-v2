use sqlx::FromRow;

/// One-to-one struct for chapter data.
#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Chapters {
    pub id: i32,
    pub chapter_name: Option<String>,
    pub is_multiplayer: bool,
    pub game_id: i32,
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

// Currently a dumbass work around to issues with deserializing an option natively theough the Query
/// Generic wrapper around an Option i32 for [actix_web::web::Query]
#[derive(Debug, Deserialize)]
pub struct OptIDs {
    pub cat_id: Option<i32>,
    pub game_id: Option<i32>,
}

/// Querying for Chapters
#[derive(Serialize, Deserialize, Debug)]
pub struct ChapterQueryParams {
    pub chapter_name: Option<String>,
    pub is_multiplayer: Option<bool>,
    pub game_id: Option<i32>,
}
