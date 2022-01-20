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
    pub id: i64,
    pub timestamp: Option<NaiveDateTime>,
    pub profile_number: String,
    pub score: i32,
    pub map_id: String,
    pub demo_id: Option<i64>,
    pub banned: bool,
    pub youtube_id: Option<String>,
    pub previous_id: Option<i32>,
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
// pub struct Changelog {
//     pub time_gained: Option<NaiveDateTime>,
//     pub profile_number: String,
//     pub score: i32,
//     pub map_id: String,
//     pub wr_gain: i32,
//     pub has_demo: Option<i32>,
//     pub banned: i32,
//     pub youtube_id: Option<String>,
//     pub previous_id: Option<i32>,
//     pub id: i32,
//     pub coopid: Option<i32>,
//     pub post_rank: Option<i32>,
//     pub pre_rank: Option<i32>,
//     pub submission: i32,
//     pub note: Option<String>,
//     pub category: Option<String>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChangelogInsert {
    pub timestamp: Option<NaiveDateTime>,
    pub profile_number: String,
    pub score: i32,
    pub map_id: String,
    pub demo_id: Option<i64>,
    pub banned: bool,
    pub youtube_id: Option<String>,
    pub previous_id: Option<i32>,
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
pub struct SpMap {
    pub timestamp: Option<NaiveDateTime>,
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

#[derive(Serialize, Deserialize)]
pub struct SpRanked {
    pub map_data: SpMap,
    pub rank: i32,
    pub points: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

/// Wrapper for the coop map data and the rank/score.
#[derive(Serialize, Deserialize)]
pub struct CoopRanked {
    pub map_data: CoopMap,
    pub rank: i32,
    pub points: f32,
}

/// To deserialize banned SP entries from the webserver API calls.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SpBanned {
    pub profile_number: String,
    pub score: i32,
}

/// To deserialize banned Coop entries from the webserver API calls.
#[derive(Deserialize, Debug)]
pub struct CoopBundled {
    pub profile_number1: String,
    pub profile_number2: Option<String>,
    pub score: i32,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct UserMap {
//     pub boardname: Option<String>,
//     pub steamname: Option<String>,
//     pub avatar: Option<String>,
// }

// /// Wrapper for a player's SP PB history.
// #[derive(Serialize, Deserialize, Clone)]
// pub struct SpPbHistory {
//     pub user_info: UserMap,
//     pub pb_history: Option<Vec<Changelog>>,
// }

/// Wrapper for a player's SP PB history.
#[derive(Serialize, Deserialize)]
pub struct SpPbHistory {
    pub user_name: Option<String>,
    pub avatar: Option<String>,
    pub pb_history: Option<Vec<Changelog>>,
}
