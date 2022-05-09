use sqlx::FromRow;

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

/// One-to-one struct for Category data.
#[derive(Serialize, Deserialize, FromRow)]
pub struct Categories {
    pub id: i32,
    pub name: String,
    pub map_id: String,
    pub rules: String,
}
