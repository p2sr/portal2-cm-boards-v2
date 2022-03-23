use super::exporting::*;
use super::uploading::*;
use crate::models::datamodels::{
    CoopDataUtil, CoopRanked, Entry, Leaderboards, SpBanned, SpRanked, Users, XmlTag,
};
use crate::LIMIT_MULT_COOP;
use crate::LIMIT_MULT_SP;
use chrono::prelude::*;
use log::{debug, error, trace};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_xml_rs::from_reader;
use std::collections::HashMap;

// TODO: If user doesn't exist, add a new user in db.

/// Grabs the map at the current ID from valve's API and caches times.
pub fn fetch_entries(
    id: i32,
    start: i32,
    end: i32,
    timestamp: NaiveDateTime,
    is_coop: bool,
) -> Leaderboards {
    let url = format!(
        "https://steamcommunity.com/stats/{game}/leaderboards/{id}?xml=1&start={start}&end={end}",
        game = "Portal2",
        id = id,
        start = start,
        end = end
    );
    let text = reqwest::blocking::get(&url)
        .expect("Error in request to valve API")
        .text()
        .expect("Error in writing the result from Valve's API to text");
    // Print to cache
    let is_updated = cache_leaderboard(id, text.clone());
    if is_updated {
        debug!("The cache is updated for map {}", id);
    }
    let leaderboard: Leaderboards = from_reader(text.as_bytes()).expect("XML Error in parsing");
    // Get banned players list.
    let banned_users: Vec<String> =
        reqwest::blocking::get("http://localhost:8080/api/v1/banned_users")
            .expect("Error in query to our local API (Make sure the webserver is running")
            .json()
            .expect("Error in converting our API values to JSON");

    match is_coop {
        false => filter_entries_sp(
            id,
            start,
            end,
            timestamp,
            banned_users,
            &leaderboard.entries,
        ),
        true => filter_entries_coop(
            id,
            start,
            end,
            timestamp,
            banned_users,
            &leaderboard.entries,
        ),
    }
    leaderboard
}

// A much lower-code implementation would be to send potential values through POST to see if they exist in the DB, but the # of db interactions would probably cause much worse performance.
/// Handles comparison with the current leaderboards to see if any user has a new best time
pub fn filter_entries_sp(
    id: i32,
    _start: i32,
    end: i32,
    timestamp: NaiveDateTime,
    banned_users: Vec<String>,
    data: &XmlTag<Vec<Entry>>,
) {
    let url = format!("http://localhost:8080/api/v1/map/sp/{id}", id = id);
    let map_json: Vec<SpRanked> = reqwest::blocking::get(&url)
        .expect("Error in query to our local API (Make sure the webserver is running")
        .json()
        .expect("Error in converting our API values to JSON");

    let mut existing_hash: HashMap<String, (i32, i32)> =
        HashMap::with_capacity((end / LIMIT_MULT_SP) as usize);
    let mut current_rank: HashMap<String, i32> = HashMap::new();
    let mut not_cheated: Vec<SpBanned> = Vec::new();

    let worst_score = map_json[(end / LIMIT_MULT_SP) as usize - 1].map_data.score;
    let wr = map_json[0].map_data.score;

    for rank in map_json.iter() {
        existing_hash.insert(
            rank.map_data.profile_number.clone(),
            (rank.map_data.score, rank.rank),
        );
    }
    // TODO: Query to see if the user exists.

    // TODO: Implement a per-map threshold???
    // Potentially turn this into a macro? This basic shape is reused.
    for entry in data.value.iter() {
        match existing_hash.get(&entry.steam_id.value) {
            Some((score, rank)) => {
                // The user has a time in top X scores currently
                if score > &entry.score.value {
                    trace!(
                        "New better time for user {} on map_id {}",
                        entry.steam_id.value,
                        id
                    ); // Add to leaderboards.
                    current_rank.insert(entry.steam_id.value.clone(), rank.clone());
                    match check_cheated(&entry.steam_id.value, &banned_users) {
                        false => not_cheated.push(SpBanned {
                            profile_number: entry.steam_id.value.clone(),
                            score: entry.score.value,
                        }),
                        _ => (),
                    }
                }
            }
            _ => {
                // The user is not currently in top X scores.
                if entry.score.value > worst_score {
                    trace!(
                        "User {} is new to top X scores on {}, we need to add their time!",
                        entry.steam_id.value,
                        id
                    );
                    match check_cheated(&entry.steam_id.value, &banned_users) {
                        false => not_cheated.push(SpBanned {
                            profile_number: entry.steam_id.value.clone(),
                            score: entry.score.value,
                        }),
                        _ => (),
                    }
                }
            }
        }
    }

    // We grab the list of banned times from our API.
    // Filter out any times that are banned from the list of potential runs.
    // The list of new scores is probably relatively low, it would be easier to just send the score information to an endpoint and have it check.
    for entry in not_cheated.iter() {
        let ban_url = format!(
            "http://localhost:8080/api/v1/sp/banned/{}?profile_number={}&score={}",
            id, entry.profile_number, entry.score
        );
        let res: bool = reqwest::blocking::get(&ban_url)
            .expect("Error in query to our local API (Make sure the webserver is running")
            .json()
            .expect("Error in converting our API values to JSON");

        match res {
            true => {
                trace!(
                    "Time {} by {} found, so time is banned. Ignore",
                    entry.score,
                    entry.profile_number
                )
            }
            false => {
                trace!(
                    "Time {} by {} not found, so assumed to be unbanned.",
                    entry.score,
                    entry.profile_number
                );
                // We have now checked that the user is not banned, that the time is top X score worthy, that the score doesn't exist in the db, but is banned.
                match post_sp_pb(
                    entry.profile_number.clone(),
                    entry.score,
                    wr,
                    id,
                    timestamp,
                    &current_rank,
                    &map_json,
                ) {
                    true => (),
                    false => error!(
                        "Time {} by {} failed to submit",
                        entry.profile_number, entry.score
                    ),
                };
            }
        }
    }
}

/// Version of `filter_entries` for coop, using different logic.
pub fn filter_entries_coop(
    id: i32,
    _start: i32,
    end: i32,
    timestamp: NaiveDateTime,
    banned_users: Vec<String>,
    data: &XmlTag<Vec<Entry>>,
) {
    let url = format!("http://localhost:8080/api/v1/map/coop/{id}", id = id);
    let map_json: Vec<CoopRanked> = reqwest::blocking::get(&url)
        .expect("Error in query to our local API (Make sure the webserver is running")
        .json()
        .expect("Error in converting our API values to JSON");

    let mut existing_hash: HashMap<String, (i32, i32)> =
        HashMap::with_capacity(((end / LIMIT_MULT_COOP) * 2) as usize);
    let worst_score = map_json[(end / LIMIT_MULT_COOP) as usize - 1]
        .map_data
        .score;
    let wr = map_json[0].map_data.score;
    let mut current_rank: HashMap<String, i32> = HashMap::new();
    let mut not_banned_player = Vec::new();
    // We attempt to insert both players into the hashmap. This way we get all players with a top X score in coop.
    for rank in map_json.iter() {
        existing_hash.insert(
            rank.map_data.profile_number1.clone(),
            (rank.map_data.score, rank.rank),
        );
        existing_hash.insert(
            rank.map_data.profile_number2.clone(),
            (rank.map_data.score, rank.rank),
        );
    }
    // Filter out all scores that exist. We filter this out in the same way that we filter SP times, the coop-specific logic is handled later
    for entry in data.value.iter() {
        match existing_hash.get(&entry.steam_id.value) {
            Some((score, rank)) => {
                // The user has a time in top X score currently
                if score > &entry.score.value {
                    trace!(
                        "New better time for user {} on map_id {}",
                        entry.steam_id.value,
                        id
                    ); // Add to leaderboards.
                    current_rank.insert(entry.steam_id.value.clone(), rank.clone());
                    match check_cheated(&entry.steam_id.value, &banned_users) {
                        // We use SpBanned here because scores taken from the SteamAPI are all handled as SP times.
                        false => not_banned_player.push(SpBanned {
                            profile_number: entry.steam_id.value.clone(),
                            score: entry.score.value,
                        }),
                        _ => (),
                    }
                }
            }
            _ => {
                // The user is not currently in top X score.
                if entry.score.value > worst_score {
                    trace!(
                        "User {} is new to top X score on {}, we need to add their time!",
                        entry.steam_id.value,
                        id
                    );
                    match check_cheated(&entry.steam_id.value, &banned_users) {
                        false => not_banned_player.push(SpBanned {
                            profile_number: entry.steam_id.value.clone(),
                            score: entry.score.value,
                        }),
                        _ => (),
                    }
                }
            }
        }
    }

    // Check to see if any of the times are banned on our leaderboards
    // Do an SP style check here first, as we want to ensure none of the times are old and banned.
    // The issue with doing this step pre-bundled would be if long-standing, banned times are bundled before checking
    // to see that they're old, banned times on the leaderboard, our assumption about all scores being new and together
    // falls apart.
    let mut not_cheated = Vec::new(); // Becomes the vector of times that are not from banned players, and do not exist in the changelog.
    for entry in not_banned_player.iter() {
        let ban_url = format!(
            "http://localhost:8080/api/v1/coop/banned/{}?profile_number={}&score={}",
            id, entry.profile_number, entry.score
        );
        let res: bool = reqwest::blocking::get(&ban_url)
            .expect("Error in query to our local API (Make sure the webserver is running")
            .json()
            .expect("Error in converting our API values to JSON");

        match res {
            true => debug!("The time was found, so the time is banned. Ignore"),
            false => not_cheated.push(entry.clone()),
        }
    }

    // The times that aren't banned should be parsed to see if there are matching times
    // If the times are matching, all old times are filtered, and no banned times are taken into consideration,
    // it's fair to assume the times were gotten together between two people
    let mut already_bundled: HashMap<String, i32> = HashMap::new();
    // Contains the bundled entries (if profile_number2 is None, there is no mathcing time)
    let mut bundled_entries = Vec::new();
    for entry in not_cheated.iter() {
        for entry2 in not_cheated.iter() {
            if (entry.profile_number != entry2.profile_number) & (entry.score == entry2.score) {
                // Scores are assumed to be gotten together.
                match already_bundled.get(&entry.profile_number) {
                    // Make sure we aren't just reading the second entry later down the line.
                    Some(_) => (),
                    None => match already_bundled.get(&entry2.profile_number) {
                        Some(_) => (),
                        None => {
                            bundled_entries.push(CoopDataUtil {
                                profile_number1: entry.profile_number.clone(),
                                profile_number2: Some(entry2.profile_number.clone()),
                                score: entry.score,
                            });
                            already_bundled.insert(entry.profile_number.clone(), 1);
                            already_bundled.insert(entry2.profile_number.clone(), 1);
                        }
                    },
                }
            }
        }
        // If we have looked through every entry, and found no match, the time is "carried" and the p2 is unknown
        match already_bundled.get(&entry.profile_number) {
            Some(_) => (),
            None => {
                bundled_entries.push(CoopDataUtil {
                    profile_number1: entry.profile_number.clone(),
                    profile_number2: None,
                    score: entry.score,
                });
                // Probably unnecessary to add to hashmap, but doing it just incase.
                already_bundled.insert(entry.profile_number.clone(), 0);
            }
        }
    }
    // Create individual changelog entries, and create a bundled coop time to represent the new times

    // Push to the database.
    for entry in bundled_entries.iter() {
        // TODO: Handle failture to insert.
        match post_coop_pb(
            entry.profile_number1.clone(),
            entry.profile_number2.clone(),
            entry.score,
            wr,
            id,
            timestamp,
            &current_rank,
            &map_json,
        ) {
            true => (),
            false => (),
        };
    }
}

pub fn check_cheated(id: &String, banned_users: &Vec<String>) -> bool {
    for entry in banned_users.iter() {
        if entry == id {
            return true;
        }
    }
    false
}

#[allow(dead_code)]
pub fn check_user(profile_number: &str) -> bool {
    let url = format!("http://localhost:8080/api/v1/users/{}", profile_number);
    let user = reqwest::blocking::get(&url)
        .expect("Error in query to our local API (Make sure the webserver is running")
        .json::<Users>();
    match user {
        Ok(_user) => true,
        Err(e) => {
            debug!("{}", e);
            false
        }
    }
}

#[allow(dead_code)]
pub fn update_image(profile_number: String) -> String {
    let api_key = dotenv::var("STEAM_API_KEY").expect("Cannot find STEAM_API_KEY in ./.env");

    let steam_api_url = format!(
        "https://api.steampowered.com/ISteamUser/GetPlayerSummaries/v2/key={}steamid={}",
        api_key, profile_number
    );
    let user = reqwest::blocking::get(&steam_api_url)
        .expect("Cannot connect to Steam API")
        .json::<GetPlayerSummariesWrapper>()
        .unwrap();
    user.response.players[0].avatarfull.clone()
}

#[allow(dead_code)]
pub fn add_user(profile_number: String) -> Users {
    // http://steamcommunity.com/profiles/{}/?xml=1
    // GET https://api.steampowered.com/ISteamUser/GetPlayerSummaries/v2/
    let api_key = dotenv::var("STEAM_API_KEY").expect("Cannot find STEAM_API_KEY in ./.env");

    let steam_api_url = format!(
        "https://api.steampowered.com/ISteamUser/GetPlayerSummaries/v2/key={}steamid={}",
        api_key, profile_number
    );
    // let delimiter = "\"avatarfull\":";
    let user = reqwest::blocking::get(&steam_api_url)
        .expect("Cannot connect to Steam API")
        .json::<GetPlayerSummariesWrapper>()
        .unwrap();

    // let slice: Vec<&str> = user.split(delimiter).collect();
    // let slice: Vec<&str> = slice[1].split(",").collect();
    // let avatar = String::from(slice[0]);
    // let steamname = String::new();

    let new_user = Users {
        profile_number,
        board_name: None,
        steam_name: Some(user.response.players[0].personaname.clone()),
        banned: false,
        registered: 0,
        avatar: Some(user.response.players[0].avatarfull.clone()),
        ..Default::default()
    };
    let url = String::from("http://localhost:8080/api/v1/users/new_user");
    let client = reqwest::blocking::Client::new();
    client
        .post(&url)
        .json(&new_user)
        .send()
        .expect("Could not post user to our internal API")
        .json::<Users>()
        .expect("Incorrect return data from our API")
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
    pub lastlogoff: String,
    pub profile_url: String,
    pub avatar: String,
    pub avatarmedium: String,
    pub avatarfull: String,
}
