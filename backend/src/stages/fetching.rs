use super::exporting::*;
use chrono::prelude::*;
use serde_xml_rs::from_reader;
use std::collections::HashMap;

use crate::models::datamodels::{
    ChangelogInsert, CoopBundled, CoopMap, CoopRanked, CoopbundledInsert, Entry, Leaderboards,
    SpBanned, SpMap, SpPbHistory, SpRanked, XmlTag,
};

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
    // TODO: Set this up with logging.
    if is_updated {
        println!("The cache is updated for map {}", id);
    }
    let leaderboard: Leaderboards = from_reader(text.as_bytes()).expect("XML Error in parsing");
    // Get banned players list.
    let banned_users: Vec<String> = reqwest::blocking::get("http://localhost:8080/api/bannedusers")
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
    start: i32,
    end: i32,
    timestamp: NaiveDateTime,
    banned_users: Vec<String>,
    data: &XmlTag<Vec<Entry>>,
) {
    let url = format!("http://localhost:8080/api/maps/sp/{id}", id = id);
    let map_json: Vec<SpRanked> = reqwest::blocking::get(&url)
        .expect("Error in query to our local API (Make sure the webserver is running")
        .json()
        .expect("Error in converting our API values to JSON");

    let mut existing_hash: HashMap<String, (i32, i32)> = HashMap::with_capacity(200);
    let mut current_rank: HashMap<String, i32> = HashMap::new();
    let mut not_cheated: Vec<SpBanned> = Vec::new();

    let worst_score = map_json[199].map_data.score;
    let wr = map_json[0].map_data.score;

    for rank in map_json.iter() {
        existing_hash.insert(
            rank.map_data.profile_number.clone(),
            (rank.map_data.score, rank.rank),
        );
    }
    // TODO: Implement a per-map threshold???
    // Potentially turn this into a macro? This basic shape is reused.
    for entry in data.value.iter() {
        match existing_hash.get(&entry.steam_id.value) {
            Some((score, rank)) => {
                // The user has a time in top 200 currently
                if score > &entry.score.value {
                    println!(
                        "New better time for user {} on map_id {}",
                        entry.steam_id.value, id
                    ); // Add to leaderboards.
                    current_rank.insert(entry.steam_id.value.clone(), rank.clone());
                    match check_cheated(&entry.steam_id.value, &banned_users) {
                        false => not_cheated.push(SpBanned {
                            profilenumber: entry.steam_id.value.clone(),
                            score: entry.score.value,
                        }),
                        _ => (),
                    }
                }
            }
            _ => {
                // The user is not currently in top 200.
                if entry.score.value > worst_score {
                    println!(
                        "User {} is new to top 200 on {}, we need to add their time!",
                        entry.steam_id.value, id
                    );
                    match check_cheated(&entry.steam_id.value, &banned_users) {
                        false => not_cheated.push(SpBanned {
                            profilenumber: entry.steam_id.value.clone(),
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
    let client = reqwest::blocking::Client::new();
    let ban_url = format!("http://localhost:8080/api/maps/sp/banned/{id}", id = id);
    for entry in not_cheated.iter() {
        let res: bool = client
            .post(&url)
            .json(entry)
            .send()
            .expect("Error querying our local API")
            .json()
            .expect("Error converting to json"); //TODO: Fix the endpoint, or change expectations
        match res {
            true => println!("The time was found, so the time is banned. Ignore"),
            false => {
                println!("Time not found, so assumed to be unbanned.");
                // We have now checked that the user is not banned, that the time is top 200 worthy, that the score doesn't exist in the db, but is banned.
                match post_sp_pb(
                    entry.profilenumber.clone(),
                    entry.score,
                    wr,
                    id,
                    timestamp,
                    &current_rank,
                    &map_json,
                ) {
                    true => (), //TODO: Handle failure.
                    false => (),
                };
            }
        }
    }
}

pub fn post_sp_pb(
    profilenumber: String,
    score: i32,
    wr: i32,
    id: i32,
    timestamp: NaiveDateTime,
    current_rank: &HashMap<String, i32>,
    map_json: &Vec<SpRanked>,
) -> bool {
    let mut wr_gain = 0;
    if score >= wr {
        wr_gain = 1;
    }
    // Grab the PB history.
    let url = format!("http://localhost:8080/api/maps/sp/{}/{}", id, profilenumber); // TODO: Handle crashing if no PB history is found.
    let pb_history: SpPbHistory = reqwest::blocking::get(&url)
        .expect("Error in query to our local API (Make sure the webserver is running")
        .json()
        .expect("Error in converting our API values to JSON");

    let mut previous_id = None;
    let pb_vec = pb_history.pb_history;
    match pb_vec {
        Some(pb_vec) => {
            let current_pb = pb_vec.into_iter().nth(0).unwrap();
            previous_id = Some(current_pb.id);
        }
        None => (),
    }

    let mut post_rank: Option<i32> = None;
    for entry in map_json.iter() {
        if entry.map_data.score == score {
            // They have the same rank
            post_rank = Some(entry.rank)
        } else if entry.map_data.score > score {
            // They will temporarily have the same rank, as when the board re-calculates, the values for the other maps will change. But this value only tracks the inital rank at time of update.
            post_rank = Some(entry.rank)
        }
    }
    let prerank: Option<i32> = match current_rank.get(&profilenumber) {
        Some(rank) => Some(rank.clone()),
        None => None,
    };

    let new_score = ChangelogInsert {
        time_gained: Some(timestamp),
        profile_number: profilenumber,
        score: score,
        map_id: id.to_string(),
        wr_gain: wr_gain,
        previous_id: previous_id, // id of last PB
        post_rank: post_rank,     // New rank as of this score update
        pre_rank: prerank,        // Rank prior to this score update
        has_demo: Some(0),
        banned: 0,
        submission: 0,
        youtube_id: None,
        coopid: None,
        note: None,
        category: Some("any%".to_string()),
    };
    let client = reqwest::blocking::Client::new();
    let post_url = "http://localhost:8080/api/sp/postscore".to_string();
    let res: bool = client
        .post(&url)
        .json(&new_score)
        .send()
        .expect("Error querying our local API")
        .json()
        .expect("Error converting to json");
    // TODO: Better handling of failure
    match res {
        true => return true,
        false => return false,
    }
}

/// Version of `filter_entries` for coop, using different logic.
pub fn filter_entries_coop(
    id: i32,
    start: i32,
    end: i32,
    timestamp: NaiveDateTime,
    banned_users: Vec<String>,
    data: &XmlTag<Vec<Entry>>,
) {
    let url = format!("http://localhost:8080/api/maps/coop/{id}", id = id);
    let map_json: Vec<CoopRanked> = reqwest::blocking::get(&url)
        .expect("Error in query to our local API (Make sure the webserver is running")
        .json()
        .expect("Error in converting our API values to JSON");

    let mut existing_hash: HashMap<String, (i32, i32)> = HashMap::with_capacity(400);
    let worst_score = map_json[199].map_data.score;
    let wr = map_json[0].map_data.score;
    let mut current_rank: HashMap<String, i32> = HashMap::new();
    let mut not_banned_player = Vec::new();
    // We attempt to insert both players into the hashmap. This way we get all players with a top 200 in coop.
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
                // The user has a time in top 200 currently
                if score > &entry.score.value {
                    println!(
                        "New better time for user {} on map_id {}",
                        entry.steam_id.value, id
                    ); // Add to leaderboards.
                    current_rank.insert(entry.steam_id.value.clone(), rank.clone());
                    match check_cheated(&entry.steam_id.value, &banned_users) {
                        // We use SpBanned here because scores taken from the SteamAPI are all handled as SP times.
                        false => not_banned_player.push(SpBanned {
                            profilenumber: entry.steam_id.value.clone(),
                            score: entry.score.value,
                        }),
                        _ => (),
                    }
                }
            }
            _ => {
                // The user is not currently in top 200.
                if entry.score.value > worst_score {
                    println!(
                        "User {} is new to top 200 on {}, we need to add their time!",
                        entry.steam_id.value, id
                    );
                    match check_cheated(&entry.steam_id.value, &banned_users) {
                        false => not_banned_player.push(SpBanned {
                            profilenumber: entry.steam_id.value.clone(),
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
    let client = reqwest::blocking::Client::new();
    let ban_url = format!("http://localhost:8080/api/maps/coop/banned/{id}", id = id);
    let mut not_cheated = Vec::new(); // Becomes the vector of times that are not from banned players, and do not exist in the changelog.
    for entry in not_banned_player.iter() {
        let res: bool = client
            .post(&url)
            .json(entry)
            .send()
            .expect("Error querying our local API")
            .json()
            .expect("Error converting to json");
        match res {
            true => println!("The time was found, so the time is banned. Ignore"),
            false => not_cheated.push(entry.clone()),
        }
    }

    // The times that aren't banned should be parsed to see if there are matching times
    // If the times are matching, all old times are filtered, and no banned times are taken into consideration,
    // it's fair to assume the times were gotten together between two people
    let mut already_bundled: HashMap<String, i32> = HashMap::new();
    // Contains the bundled entries (if profilenumber2 is None, there is no mathcing time)
    let mut bundled_entries = Vec::new();
    for entry in not_cheated.iter() {
        for entry2 in not_cheated.iter() {
            if (entry.profilenumber != entry2.profilenumber) & (entry.score == entry2.score) {
                // Scores are assumed to be gotten together.
                match already_bundled.get(&entry.profilenumber) {
                    // Make sure we aren't just reading the second entry later down the line.
                    Some(_) => (),
                    None => match already_bundled.get(&entry2.profilenumber) {
                        Some(_) => (),
                        None => {
                            bundled_entries.push(CoopBundled {
                                profilenumber1: entry.profilenumber.clone(),
                                profilenumber2: Some(entry2.profilenumber.clone()),
                                score: entry.score,
                            });
                            already_bundled.insert(entry.profilenumber.clone(), 1);
                            already_bundled.insert(entry2.profilenumber.clone(), 1);
                        }
                    },
                }
            }
        }
        // If we have looked through every entry, and found no match, the time is "carried" and the p2 is unknown
        match already_bundled.get(&entry.profilenumber) {
            Some(_) => (),
            None => {
                bundled_entries.push(CoopBundled {
                    profilenumber1: entry.profilenumber.clone(),
                    profilenumber2: None,
                    score: entry.score,
                });
                // Probably unnecessary to add to hashmap, but doing it just incase.
                already_bundled.insert(entry.profilenumber.clone(), 0);
            }
        }
    }
    // Create individual changelog entries, and create a bundled coop time to represent the new times

    // Push to the database.
    for entry in bundled_entries.iter() {
        // TODO: Handle failture to insert.
        match post_coop_pb(
            entry.profilenumber1.clone(),
            entry.profilenumber2.clone(),
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

///
pub fn post_coop_pb(
    profilenumber1: String,
    profilenumber2: Option<String>,
    score: i32,
    wr: i32,
    id: i32,
    timestamp: NaiveDateTime,
    current_rank: &HashMap<String, i32>,
    map_json: &Vec<CoopRanked>,
) -> bool {
    let mut wr_gain = 0;
    if score >= wr {
        wr_gain = 1;
    }
    // Handle there being a partner
    if let Some(profilenumber2) = profilenumber2 {
        // Grab the PB history. For now, we're just going to use 2 calls to our API rather than a combined call. (We'll use SP here).
        let url = format!(
            "http://localhost:8080/api/maps/sp/{}/{}",
            id, profilenumber1
        ); // TODO: Handle crashing if no PB history is found.
        let pb_history1: SpPbHistory = reqwest::blocking::get(&url)
            .expect("Error in query to our local API (Make sure the webserver is running")
            .json()
            .expect("Error in converting our API values to JSON");
        let url = format!(
            "http://localhost:8080/api/maps/sp/{}/{}",
            id, profilenumber2
        ); // TODO: Handle crashing if no PB history is found.
        let pb_history2: SpPbHistory = reqwest::blocking::get(&url)
            .expect("Error in query to our local API (Make sure the webserver is running")
            .json()
            .expect("Error in converting our API values to JSON");
        let mut previous_id1 = None;
        let pb_vec = pb_history1.pb_history;
        match pb_vec {
            Some(pb_vec) => {
                let current_pb = pb_vec.into_iter().nth(0).unwrap();
                previous_id1 = Some(current_pb.id);
            }
            None => (),
        }
        let mut previous_id2 = None;
        let pb_vec = pb_history2.pb_history;
        match pb_vec {
            Some(pb_vec) => {
                let current_pb = pb_vec.into_iter().nth(0).unwrap();
                previous_id2 = Some(current_pb.id);
            }
            None => (),
        }

        let mut post_rank: Option<i32> = None;
        for entry in map_json.iter() {
            if entry.map_data.score == score {
                // They have the same rank
                post_rank = Some(entry.rank)
            } else if entry.map_data.score > score {
                // They will temporarily have the same rank, as when the board re-calculates, the values for the other maps will change. But this value only tracks the inital rank at time of update.
                post_rank = Some(entry.rank)
            }
        }

        let prerank1: Option<i32> = match current_rank.get(&profilenumber1) {
            Some(rank) => Some(rank.clone()),
            None => None,
        };
        let prerank2: Option<i32> = match current_rank.get(&profilenumber2) {
            Some(rank) => Some(rank.clone()),
            None => None,
        };

        // TODO: We first need to upload individually as changelog entries, get the result from that insert (the changelogID it creates, then use that for the bundling process).

        let news_score = CoopbundledInsert {
            time_gained: Some(timestamp),
            profile_number1: profilenumber1,
            profile_number2: profilenumber2,
            score: score,
            map_id: id.to_string(),
            wr_gain: wr_gain,
            is_blue: None,
            has_demo1: Some(0),
            has_demo2: Some(0),
            banned: 0,
            youtube_id1: None,
            youtube_id2: None,
            previous_id1: previous_id1,
            previous_id2: previous_id2,
            changelogid1: 0,       //TODO: Get clid
            changelogid2: 0,       //TODO: Get clid
            post_rank1: post_rank, //TODO: We should not have 2 post_ranks. The values should always be the same
            post_rank2: post_rank,
            pre_rank1: prerank1,
            pre_rank2: prerank2,
            submission1: 0,
            submission2: 0,
            note1: None,
            note2: None,
            category: Some("any%".to_string()),
        };
    }

    true
}

pub fn check_cheated(id: &String, banned_users: &Vec<String>) -> bool {
    for entry in banned_users.iter() {
        if entry == id {
            return true;
        }
    }
    false
}
