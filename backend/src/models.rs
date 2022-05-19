#![allow(non_snake_case)]
use anyhow::Result;
use chrono::NaiveDateTime;
use std::collections::HashMap;

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
    pub previous_id: Option<i64>,
    pub coop_id: Option<i64>,
    pub post_rank: Option<i32>,
    pub pre_rank: Option<i32>,
    pub submission: i32,
    pub note: Option<String>,
    pub category_id: i32,
    pub score_delta: Option<i32>,
    pub verified: Option<bool>,
    pub admin_note: Option<String>,
    pub updated: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    pub submission: i32,
    pub note: Option<String>,
    pub category_id: i32,
    pub score_delta: Option<i32>,
    pub verified: Option<bool>,
    pub admin_note: Option<String>,
}

/// One-to-one struct for coop_bundled data.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoopBundled {
    pub id: i64,
    pub p_id1: String,
    pub p_id2: Option<String>,
    pub p1_is_host: Option<bool>,
    pub cl_id1: i64,
    pub cl_id2: Option<i64>,
    pub updated: Option<NaiveDateTime>,
}

/// Insert struct for creating a new `CoopBundled`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoopBundledInsert {
    pub p_id1: String,
    pub p_id2: Option<String>,
    pub p1_is_host: Option<bool>,
    pub cl_id1: i64,
    pub cl_id2: Option<i64>,
}

/// Values that we return after checking if a score is valid to be added to the database.
#[derive(Debug, Serialize, Deserialize)]
pub struct CalcValues {
    pub previous_id: Option<i64>,
    pub post_rank: Option<i32>,
    pub pre_rank: Option<i32>,
    pub score_delta: Option<i32>,
    pub banned: bool,
}

/// Insert struct for `Demos`, excludes `id`
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DemoInsert {
    pub file_id: String,
    pub partner_name: Option<String>,
    pub parsed_successfully: bool,
    pub sar_version: Option<String>,
    pub cl_id: i64,
}
/// Allows us to upload both a demo_id and a changelog id to the webserver.
#[derive(Debug, Serialize)]
pub struct DemoOptions {
    pub demo_id: i64,
    pub cl_id: i64,
}

/// This struct handles the minimal information we want for SP map pages. We want to limit the amount of data we need to transfer.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpMap {
    pub timestamp: Option<NaiveDateTime>,
    pub profile_number: String,
    pub score: i32,
    pub demo_id: Option<i64>,
    pub youtube_id: Option<String>,
    pub submission: i32,
    pub note: Option<String>,
    pub category_id: i32,
    pub user_name: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
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

/// Wrapper for the coop map data and the rank/score.
#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
pub struct CoopDataUtil {
    pub profile_number1: String,
    pub profile_number2: Option<String>,
    pub score: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CoopTempUser {
    pub cl_id: i64,
    pub profile_number: String,
}

/// Wrapper for a player's SP PB history.
#[derive(Serialize, Deserialize)]
pub struct SpPbHistory {
    pub user_name: Option<String>,
    pub avatar: Option<String>,
    pub pb_history: Option<Vec<Changelog>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
    pub auth_hash: Option<String>,
    pub country_id: Option<i32>,
}

/// Wrapper for our API call
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetPlayerSummariesWrapper {
    pub response: Players,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Players {
    pub players: Vec<GetPlayerSummaries>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetPlayerSummaries {
    pub steamid: String,
    pub communityvisibilitystate: i32,
    pub profilestate: i32,
    pub personaname: String,
    pub lastlogoff: i32,
    pub profileurl: String,
    pub avatar: String,
    pub avatarmedium: String,
    pub avatarfull: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct PostSP<'a> {
    pub profile_number: String,
    pub score: i32,
    pub id: i32,
    pub timestamp: NaiveDateTime,
    pub current_rank: &'a HashMap<String, i32>,
    pub map_json: &'a [SpRanked],
    pub cat_id: i32,
}

#[derive(Serialize, Debug, Clone)]
pub struct PostCoop<'a> {
    pub profile_number1: String,
    pub profile_number2: Option<String>,
    pub score: i32,
    pub id: i32,
    pub timestamp: NaiveDateTime,
    pub current_rank: &'a HashMap<String, i32>,
    pub map_json: &'a [CoopRanked],
    pub cat_id: i32,
}

#[derive(Clone, Debug, Serialize)]
pub struct AvatarInsert<'a> {
    pub avatar: &'a str,
}

#[derive(Debug, Clone)]
pub struct FetchingData {
    pub id: i32,
    pub start: i32,
    pub end: i32,
    pub timestamp: NaiveDateTime,
    pub banned_users: Vec<String>,
    pub is_coop: bool,
    pub cat_id: i32,
}

/// Legacy
pub struct UserData {
    pub displayName: String,
    pub profile_number: String,
    pub boardname: String, // can be an empty string ""
    pub steamname: String,
    pub banned: i32,
    pub registered: i32,
    pub avatar: String,
    pub twitch: String,
    pub youtube: String,
    pub title: Option<String>,
    pub admin: i32,
    pub donation_amount: Option<i32>,
}

/// Legacy
#[derive(Deserialize, Serialize, Debug)]
pub struct LegacyBoardEntry {
    player_name: String,
    avatar: String,
    profile_number: String,
    score: String,
    id: String,
    pre_rank: Option<String>,
    post_rank: Option<String>,
    wr_gain: String,
    time_gained: String,
    hasDemo: String,
    youtubeID: Option<String>,
    note: Option<String>,
    banned: String,
    submission: String,
    pending: String,
    previous_score: Option<String>,
    chamberName: String,
    pub chapterId: String,
    pub mapid: String,
    improvement: Option<i32>,
    rank_improvement: Option<i32>,
    pre_points: Option<String>,
    post_point: Option<String>,
    point_improvement: Option<String>,
}

// pub struct ChangelogSearch<'a> {
//     pub profile_number: &'a str,
//     pub map_id: &'a str,
//     pub score: i32,
// }

impl LegacyBoardEntry {
    pub async fn convert_to_changelog(
        self,
        cat: i32,
    ) -> Option<(ChangelogInsert, Option<DemoInsert>)> {
        if let Some(validation_details) =
            LegacyBoardEntry::check_valid(&self.profile_number, &self.mapid, &self.score).await
        {
            let score_delta = self.improvement.map(|x| -x);
            let cl = ChangelogInsert {
                timestamp: Some(
                    chrono::NaiveDateTime::parse_from_str(&self.time_gained, "%Y-%m-%d %H:%M:%S")
                        .unwrap(),
                ),
                profile_number: self.profile_number.clone(),
                score: self.score.parse().unwrap(),
                map_id: self.mapid,
                demo_id: None,
                banned: LegacyBoardEntry::bool_from_str(&self.banned),
                youtube_id: self.youtubeID,
                previous_id: validation_details.previous_id,
                coop_id: None,
                post_rank: self.post_rank.map(|x| x.parse().unwrap()),
                pre_rank: self.pre_rank.map(|x| x.parse().unwrap()),
                submission: self.submission.parse().unwrap(),
                note: self.note,
                category_id: cat,
                score_delta,
                verified: Some(!LegacyBoardEntry::bool_from_str(&self.pending)),
                admin_note: None,
            };
            let demo = if &self.hasDemo == "1" {
                // Create a demo.
                Some(DemoInsert {
                    cl_id: self.id.parse().unwrap(),
                    // This is different from the ones we get from the python script, we don't include the ID.
                    file_id: format!(
                        "{}_{}_{}",
                        self.chamberName.replace(' ', ""),
                        self.score,
                        self.profile_number.clone()
                    ),
                    partner_name: None,
                    parsed_successfully: true,
                    sar_version: None,
                })
            } else {
                None
            };
            Some((cl, demo))
        } else {
            // Check to see if the user exists, if they don't, add the user.
            match crate::stages::uploading::add_user(&self.profile_number).await {
                Ok(_user) => {
                    if let Some(validation_details) = LegacyBoardEntry::check_valid(
                        &self.profile_number,
                        &self.mapid,
                        &self.score,
                    )
                    .await
                    {
                        let score_delta = self.improvement.map(|x| -x);
                        let cl = ChangelogInsert {
                            timestamp: Some(
                                chrono::NaiveDateTime::parse_from_str(
                                    &self.time_gained,
                                    "%Y-%m-%d %H:%M:%S",
                                )
                                .unwrap(),
                            ),
                            profile_number: self.profile_number.clone(),
                            score: self.score.parse().unwrap(),
                            map_id: self.mapid,
                            demo_id: None,
                            banned: LegacyBoardEntry::bool_from_str(&self.banned),
                            youtube_id: self.youtubeID,
                            previous_id: validation_details.previous_id,
                            coop_id: None,
                            post_rank: self.post_rank.map(|x| x.parse().unwrap()),
                            pre_rank: self.pre_rank.map(|x| x.parse().unwrap()),
                            submission: self.submission.parse().unwrap(),
                            note: self.note,
                            category_id: cat,
                            score_delta,
                            verified: Some(!LegacyBoardEntry::bool_from_str(&self.pending)),
                            admin_note: None,
                        };
                        let demo = if &self.hasDemo == "1" {
                            // Create a demo.
                            Some(DemoInsert {
                                cl_id: self.id.parse().unwrap(),
                                // This is different from the ones we get from the python script, we don't include the ID.
                                file_id: format!(
                                    "{}_{}_{}",
                                    self.chamberName.replace(' ', ""),
                                    self.score,
                                    self.profile_number.clone()
                                ),
                                partner_name: None,
                                parsed_successfully: true,
                                sar_version: None,
                            })
                        } else {
                            None
                        };
                        Some((cl, demo))
                    } else {
                        None
                    }
                }
                Err(_e) => None,
            }
        }
    }
    fn bool_from_str(banned: &str) -> bool {
        match banned {
            "0" => false,
            "1" => true,
            _ => true,
        }
    }
    async fn check_valid(profile_number: &str, map_id: &str, score: &str) -> Option<CalcValues> {
        let url = format!(
            "http://localhost:8080/api/v1/sp/validate?profile_number={}&score={}&map_id={}",
            profile_number, score, map_id
        );
        match reqwest::get(&url).await
            .expect("Error calling the p2boards API, be sure the server is running locally in dev more, or you can access the remote endpoint.")
            .json::<CalcValues>().await {
                Ok(res) => Some(res),
                Err(_e) => None,
            }
    }
}

// pub p_id1: String,
// pub p_id2: Option<String>,
// pub p1_is_host: Option<bool>,
// pub cl_id1: i64,
// pub cl_id2: Option<i64>,

impl CoopBundledInsert {
    pub async fn create_from_single(p_id: String, cl_id: i64, map_id: &str) -> CoopBundledInsert {
        let res: CoopTempUser = CoopBundledInsert::get_temp_user(map_id).await.unwrap();
        CoopBundledInsert {
            p_id1: p_id,
            p_id2: Some(res.profile_number),
            p1_is_host: None,
            cl_id1: cl_id,
            cl_id2: Some(res.cl_id),
        }
    }
    pub async fn get_temp_user(map_id: &str) -> Result<CoopTempUser> {
        let url = format!("http://localhost:8080/api/v1/coop/temp/{map_id}");
        Ok(reqwest::get(&url).await
            .expect("Error calling the p2boards API, be sure the server is running locally in dev more, or you can access the remote endpoint.")
            .json::<CoopTempUser>().await?)
    }
}
