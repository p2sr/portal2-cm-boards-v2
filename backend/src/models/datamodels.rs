use chrono::NaiveDateTime;

#[derive(Debug, Deserialize)]
pub struct XmlTag<T> {
    #[serde(rename = "$value")]
    pub value: T,
}

#[derive(Debug, Deserialize)]
pub struct Entry {
    #[serde(rename = "steamid")]
    pub steam_id: XmlTag<String>,
    pub score: XmlTag<i32>,
}

#[derive(Debug, Deserialize)]
pub struct Leaderboards {
    #[serde(rename = "resultCount")]
    pub result_count: XmlTag<i32>,
    pub entries: XmlTag<Vec<Entry>>,
}

/// This struct handles the minimal information we want for SP map pages. We want to limit the amount of data we need to transfer.
#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize)]
pub struct SPRanked{
    pub map_data: SPMap,
    pub rank: i32,
    pub score: f32,
}