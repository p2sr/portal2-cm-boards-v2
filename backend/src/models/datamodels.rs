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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Changelog {
    pub time_gained: Option<NaiveDateTime>,
    pub profile_number: String,
    pub score: i32,
    pub map_id: String,
    pub wr_gain: i32,
    pub has_demo: Option<i32>,
    pub banned: i32,
    pub youtube_id: Option<String>,
    pub previous_id: Option<i32>,
    pub id: i32,
    pub coopid: Option<i32>,
    pub post_rank: Option<i32>,
    pub pre_rank: Option<i32>,
    pub submission: i32,
    pub note: Option<String>,
    pub category: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChangelogInsert {
    pub time_gained: Option<NaiveDateTime>,
    pub profile_number: String,
    pub score: i32,
    pub map_id: String,
    pub wr_gain: i32,
    pub has_demo: Option<i32>,
    pub banned: i32,
    pub youtube_id: Option<String>,
    pub previous_id: Option<i32>,
    pub coopid: Option<i32>,
    pub post_rank: Option<i32>,
    pub pre_rank: Option<i32>,
    pub submission: i32,
    pub note: Option<String>,
    pub category: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoopbundledInsert {
    pub time_gained: Option<NaiveDateTime>,
    pub profile_number1: String,
    pub profile_number2: String,
    pub score: i32,
    pub map_id: String,
    pub wr_gain: i32,
    pub is_blue: Option<i32>,
    pub has_demo1: Option<i32>,
    pub has_demo2: Option<i32>,
    pub banned: i32,
    pub youtube_id1: Option<String>,
    pub youtube_id2: Option<String>,
    pub previous_id1: Option<i32>,
    pub previous_id2: Option<i32>,
    pub changelogid1: i32,
    pub changelogid2: i32,
    pub post_rank1: Option<i32>,
    pub post_rank2: Option<i32>,
    pub pre_rank1: Option<i32>,
    pub pre_rank2: Option<i32>,
    pub submission1: i32,
    pub submission2: i32,
    pub note1: Option<String>,
    pub note2: Option<String>,
    pub category: Option<String>,
}


/// This struct handles the minimal information we want for SP map pages. We want to limit the amount of data we need to transfer.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpMap{
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
pub struct SpRanked{
    pub map_data: SpMap,
    pub rank: i32,
    pub score: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoopMap{
    pub time_gained: Option<NaiveDateTime>,
    pub profile_number1: String,
    pub profile_number2: String,
    pub score: i32,
    pub is_blue: Option<i32>,
    pub has_demo1: Option<i32>,
    pub has_demo2: Option<i32>,
    pub youtube_id1: Option<String>,
    pub youtube_id2: Option<String>,
    pub submission1: i32,
    pub submission2: i32,
    pub note1: Option<String>,
    pub note2: Option<String>,
    pub category: Option<String>,
    pub boardname1: Option<String>,
    pub steamname1: Option<String>,
    pub avatar1: Option<String>,
    pub boardname2: Option<String>,
    pub steamname2: Option<String>,
    pub avatar2: Option<String>,
}

/// Wrapper for the coop map data and the rank/score.
#[derive(Serialize, Deserialize)]
pub struct CoopRanked{
    pub map_data: CoopMap,
    pub rank: i32,
    pub score: f32,
}

/// To deserialize banned SP entries from the webserver API calls.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SpBanned{
    pub profilenumber: String,
    pub score: i32,
}

/// To deserialize banned Coop entries from the webserver API calls.
#[derive(Deserialize, Debug)]
pub struct CoopBundled{
    pub profilenumber1: String,
    pub profilenumber2: Option<String>,
    pub score: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserMap{
    pub boardname: Option<String>,
    pub steamname: Option<String>,
    pub avatar: Option<String>,
}

/// Wrapper for a player's SP PB history.
#[derive(Serialize, Deserialize, Clone)]
pub struct SpPbHistory{
    pub user_info: UserMap,
    pub pb_history: Option<Vec<Changelog>>,
}
