use super::exporting::*;
use chrono::prelude::*;
use serde_xml_rs::from_reader;
use std::collections::HashMap;
use log::{debug, trace, error};

use crate::models::datamodels::{
    ChangelogInsert, CoopBundled, CoopMap, CoopRanked, CoopBundledInsert, Entry, Leaderboards,
    SpBanned, SpMap, SpPbHistory, SpRanked, XmlTag, CoopDataUtil
};

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
    let banned_users: Vec<String> = reqwest::blocking::get("http://localhost:8080/api/banned_users")
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
                    trace!(
                        "New better time for user {} on map_id {}",
                        entry.steam_id.value, id
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
                // The user is not currently in top 200.
                if entry.score.value > worst_score {
                    trace!(
                        "User {} is new to top 200 on {}, we need to add their time!",
                        entry.steam_id.value, id
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
    let client = reqwest::blocking::Client::new();
    let ban_url = format!("http://localhost:8080/api/maps/sp/banned/{id}", id = id);
    for entry in not_cheated.iter() {
        let res: bool = client
            .post(&ban_url)
            .json(entry)
            .send()
            .expect("Error querying our local API")
            .json()
            .expect("Error converting to json");
        match res {
            true => {
                trace!("Time {} by {} found, so time is banned. Ignore", entry.score, entry.profile_number)
            },
            false => {
                trace!("Time {} by {} not found, so assumed to be unbanned.", entry.score, entry.profile_number);
                // We have now checked that the user is not banned, that the time is top 200 worthy, that the score doesn't exist in the db, but is banned.
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
                    false => error!("Time {} by {} failed to submit", entry.profile_number, entry.score),
                };
            }
        }
    }
}

pub fn post_sp_pb(
    profile_number: String,
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
    let url = format!(
        "http://localhost:8080/api/maps/sp/{}/{}",
        id, profile_number
    );
    let res = reqwest::blocking::get(&url)
        .expect("Error in query to our local API (Make sure the webserver is running")
        .json::<SpPbHistory>();
    let pb_history = match res {
        Ok(s) =>  s,
        Err(e) => {
            trace!("{}", e);
            SpPbHistory{user_name: None, avatar: None, pb_history: None}
        },
    };

    let mut previous_id = None;
    let pb_vec = pb_history.pb_history;
    let mut past_score: Option<i32> = None;
    match pb_vec {
        Some(pb_vec) => {
            let current_pb = pb_vec.into_iter().nth(0);
            if let Some(s) = current_pb {
                let current_pb = s;
                previous_id = Some(current_pb.id as i32);
                past_score = Some(current_pb.score);
            } else {
                previous_id = None;
                past_score = None;
            }
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
    let prerank: Option<i32> = match current_rank.get(&profile_number) {
        Some(rank) => Some(rank.clone()),
        None => None,
    };
    let mut score_delta: Option<i32> = None;
    if let Some(i) = past_score {
        score_delta = Some(score-i);
    }
    let mut cat_id = 0;
    let url = format!("http://localhost:8080/api/category/default_category/{}", id);
    let res = reqwest::blocking::get(&url)
        .expect("Error in query to our local API (Make sure the webserver is running")
        .json::<i32>();
    let pb_history = match res {
        Ok(s) =>  cat_id = s,
        Err(e) => {
            trace!("{}", e);
        },
    };
    let new_score = ChangelogInsert {
        timestamp: Some(timestamp),
        profile_number: profile_number,
        score: score,
        map_id: id.to_string(),
        demo_id: None,
        banned: false,
        youtube_id: None,
        previous_id: previous_id, // id of last PB
        coop_id: None,
        post_rank: post_rank,     // New rank as of this score update
        pre_rank: prerank,        // Rank prior to this score update
        submission: false,
        note: None,
        category_id: cat_id,
        score_delta: score_delta,
        verified: None,
        admin_note: None,
    };
    let client = reqwest::blocking::Client::new();
    let post_url = "http://localhost:8080/api/sp/post_score".to_string();
    let res = client
        .post(&post_url)
        .json(&new_score)
        .send()
        .expect("Error querying our local API")
        .json::<i64>();
    match res {
        Ok(s) => {
            trace!("{}", s)
        },
        Err(e) => {
            error!("{}", e);
            return false
        },
    }
    true
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
                    trace!(
                        "New better time for user {} on map_id {}",
                        entry.steam_id.value, id
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
                // The user is not currently in top 200.
                if entry.score.value > worst_score {
                    trace!(
                        "User {} is new to top 200 on {}, we need to add their time!",
                        entry.steam_id.value, id
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
    let client = reqwest::blocking::Client::new();
    let ban_url = format!("http://localhost:8080/api/maps/coop/banned/{id}", id = id);
    let mut not_cheated = Vec::new(); // Becomes the vector of times that are not from banned players, and do not exist in the changelog.
    for entry in not_banned_player.iter() {
        let res: bool = client
            .post(&ban_url)
            .json(entry)
            .send()
            .expect("Error querying our local API")
            .json()
            .expect("Error converting to json");
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

///
pub fn post_coop_pb(
    profile_number1: String,
    profile_number2: Option<String>,
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
    if let Some(profile_number2) = profile_number2 {
        // Grab the PB history. For now, we're just going to use 2 calls to our API rather than a combined call. (We'll use SP here).
        let url = format!(
            "http://localhost:8080/api/maps/sp/{}/{}",
            id, profile_number1
        ); // TODO: Handle crashing if no PB history is found.
        let pb_history1: SpPbHistory = reqwest::blocking::get(&url)
            .expect("Error in query to our local API (Make sure the webserver is running")
            .json()
            .expect("Error in converting our API values to JSON");
        let url = format!(
            "http://localhost:8080/api/maps/sp/{}/{}",
            id, profile_number2
        ); // TODO: Handle crashing if no PB history is found.
        let pb_history2: SpPbHistory = reqwest::blocking::get(&url)
            .expect("Error in query to our local API (Make sure the webserver is running")
            .json()
            .expect("Error in converting our API values to JSON");
        let mut previous_id1 = None;
        // TODO: Fix to handle new pb_history
        // TODO: Make specific to coop
        let pb_vec = pb_history1.pb_history;
        let mut past_score: Option<i32> = None;
        match pb_vec {
            Some(pb_vec) => {
                let current_pb = pb_vec.into_iter().nth(0);
                if let Some(s) = current_pb {
                    let current_pb = s;
                    previous_id1 = Some(current_pb.id as i32);
                    past_score = Some(current_pb.score);
                } else {
                    previous_id1 = None;
                    past_score = None;
                }
            }
            None => (),
        }
        let mut previous_id2 = None;
        let pb_vec = pb_history2.pb_history;
        let mut past_score: Option<i32> = None;
        match pb_vec {
            Some(pb_vec) => {
                let current_pb = pb_vec.into_iter().nth(0);
                if let Some(s) = current_pb {
                    let current_pb = s;
                    previous_id2 = Some(current_pb.id as i32);
                    past_score = Some(current_pb.score);
                } else {
                    previous_id2 = None;
                    past_score = None;
                }
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

        let prerank1: Option<i32> = match current_rank.get(&profile_number1) {
            Some(rank) => Some(rank.clone()),
            None => None,
        };
        let prerank2: Option<i32> = match current_rank.get(&profile_number2) {
            Some(rank) => Some(rank.clone()),
            None => None,
        };
        let mut score_delta1: Option<i32> = None;
        if let Some(i) = past_score {
            score_delta1 = Some(score-i);
        }
        let mut score_delta2: Option<i32> = None;
        if let Some(i) = past_score {
            score_delta2 = Some(score-i);
        }
        let mut cat_id = 0;
        
        let url = format!("http://localhost:8080/api/category/default_category/{}", id);
        let res = reqwest::blocking::get(&url)
            .expect("Error in query to our local API (Make sure the webserver is running")
            .json::<i32>();
        let pb_history = match res {
            Ok(s) =>  cat_id = s,
            Err(e) => {
                trace!("{}", e);
            },
        };
        //println!("{}", cat_id);

        // TODO: We first need to upload individually as changelog entries, get the result from that insert (the changelogID it creates, then use that for the bundling process).
        // TODO: Getting 404s on all post calls
        let score1 = ChangelogInsert { 
            timestamp: Some(timestamp),
            profile_number: profile_number1.clone(),
            score: score,
            map_id: id.to_string(),
            demo_id: None,
            banned: false,
            youtube_id: None,
            previous_id: previous_id1, // id of last PB
            coop_id: None,
            post_rank: post_rank,     // New rank as of this score update
            pre_rank: prerank1,        // Rank prior to this score update
            submission: false,
            note: None,
            category_id: cat_id,
            score_delta: score_delta1,
            verified: None,
            admin_note: None,
        };

        let score2 = ChangelogInsert {
            timestamp: Some(timestamp),
            profile_number: profile_number2.clone(),
            score: score,
            map_id: id.to_string(),
            demo_id: None,
            banned: false,
            youtube_id: None,
            previous_id: previous_id2, // id of last PB
            coop_id: None,
            post_rank: post_rank,     // New rank as of this score update
            pre_rank: prerank2,        // Rank prior to this score update
            submission: false,
            note: None,
            category_id: cat_id,
            score_delta: score_delta2,
            verified: None,
            admin_note: None,
        };
        debug!("{:#?}", score1);

        debug!("{:#?}", score2);
        // Insert both changelog entries, retrieve their IDs, create bundle
        
        let client = reqwest::blocking::Client::new();
        let mut new_id1 = 0;
        let mut new_id2 = 0;
        let post_url = "http://localhost:8080/api/sp/post_score".to_string();
        let res = client
            .post(&post_url)
            .json(&score1)
            .send()
            .expect("Error querying our local API")
            .json::<i64>();
        match res {
            Ok(s) => {
                new_id1 = s;
            },
            Err(e) => {
                error!("{}", e)
            },
        }
        let res = client
            .post(&post_url)
            .json(&score2)
            .send()
            .expect("Error querying our local API")
            .json::<i64>();
        match res {
            Ok(s) => {
                new_id2 = s;
            },
            Err(e) => {
                error!("{}", e)
            },
        }
        let bundle = CoopBundledInsert{
            p_id1: profile_number1,
            p_id2: Some(profile_number2),
            p1_is_host: None,
            cl_id1: new_id1,
            cl_id2: Some(new_id2),
        };
        let post_url = "http://localhost:8080/api/coop/post_score".to_string();
        let res = client
            .post(&post_url)
            .json(&bundle)
            .send()
            .expect("Error querying our local API")
            .json::<i64>();
        match res {
            Ok(s) => {
                trace!("{}", s);
            },
            Err(e) => {
                debug!("{}", e);
            }
        }
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
