use chrono::NaiveDateTime;
use sqlx::FromRow;

/// One-to-one mapping for badges.
#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct Badges {
    pub id: i32,
    pub name: String,
    pub image: String,
    pub description: String,
    pub tier: i32,
}

/// One-to-one mapping for profile badge entries.
#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct BadgeEntries {
    pub id: i32,
    pub badge_id: i32,
    pub profile_number: String,
    pub note: Option<String>,
    pub timestamp: Option<NaiveDateTime>,
    pub updated: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BadgeInsert {
    pub name: String,
    pub image: String,
    pub description: String,
    pub tier: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BadgeEntryInsert {
    pub badge_id: i32,
    pub profile_number: String,
    pub note: Option<String>,
    pub timestamp: Option<NaiveDateTime>,
    pub updated: Option<NaiveDateTime>,
}
