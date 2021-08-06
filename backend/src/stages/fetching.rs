use std::collections::HashMap;
use serde_xml_rs::from_reader;
use super::exporting::*;
use chrono::prelude::*;

use crate::models::datamodels::{SpMap, SpPbHistory, SpRanked, SpBanned, Changelog, CoopBanned, CoopMap, CoopRanked, Leaderboards, XmlTag, Entry};

/// Grabs the map at the current ID from valve's API and caches times.
pub fn fetch_entries(id: i32, start: i32, end: i32, timestamp: NaiveDateTime, is_coop: bool) -> Leaderboards {
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
    if is_updated{
        println!("The cache is updated for map {}", id);
    }
    let leaderboard: Leaderboards = from_reader(text.as_bytes())
        .expect("XML Error in parsing");
    
    // Get banned players list.
    let banned_users: Vec<String> = reqwest::blocking::get("http://localhost:8080/api/bannedusers")
        .expect("Error in query to our local API (Make sure the webserver is running")
        .json()
        .expect("Error in converting our API values to JSON");
    
    match is_coop{
        false => filter_entries_sp(id, start, end, timestamp, banned_users, &leaderboard.entries),
        true => filter_entries_coop(id, start, end, timestamp, banned_users, &leaderboard.entries),
    }
    leaderboard
}

// A much lower-code implementation would be to send potential values through POST to see if they exist in the DB, but the # of db interactions would probably cause much worse performance.
/// Handles comparison with the current leaderboards to see if any user has a new best time
pub fn filter_entries_sp(id: i32, start: i32, end: i32, timestamp: NaiveDateTime, banned_users: Vec<String>, data: &XmlTag<Vec<Entry>>){
    let url = format!("http://localhost:8080/api/maps/sp/{id}", id = id);
    let map_json: Vec<SpRanked> = reqwest::blocking::get(&url)
        .expect("Error in query to our local API (Make sure the webserver is running")
        .json()
        .expect("Error in converting our API values to JSON");

    let mut existing_hash: HashMap<String, (i32, i32)> = HashMap::with_capacity(200);
    let mut not_cheated: Vec<SpBanned> = Vec::new();

    let mut current_rank: Option<i32> = None;
    let worst_score = map_json[199].map_data.score;
    let wr = map_json[0].map_data.score;

    for rank in map_json.iter(){
        existing_hash.insert(rank.map_data.profile_number.clone(), (rank.map_data.score, rank.rank));
    } 
    // TODO: Implement a per-map threshold???
    // Potentially turn this into a macro? This basic shape is reused.
    for entry in data.value.iter(){
        match existing_hash.get(&entry.steam_id.value){
            Some((score, rank)) => {    // The user has a time in top 200 currently
                if score > &entry.score.value{
                    println!("New better time for user {} on map_id {}", entry.steam_id.value, id); // Add to leaderboards.
                    current_rank = Some(rank.clone());  // Save the rank.
                    match check_cheated(&entry.steam_id.value, &banned_users) {
                        false => not_cheated.push(SpBanned {profilenumber: entry.steam_id.value.clone() , score: entry.score.value}),
                        _ => (),
                    }
                }
            } _ => { // The user is not currently in top 200.
                if entry.score.value > worst_score{
                    println!("User {} is new to top 200 on {}, we need to add their time!", entry.steam_id.value, id);
                    match check_cheated(&entry.steam_id.value, &banned_users) {
                        false => not_cheated.push(SpBanned {profilenumber: entry.steam_id.value.clone() , score: entry.score.value}),
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
    for entry in not_cheated.iter(){
        let res: bool = client
            .post(&url)
            .json(entry)
            .send()
            .expect("Error querying our local API")
            .json()
            .expect("Error converting to json");
        match res{
            true => println!("The time was found, so the time is banned. Ignore"),
            false => {
                println!("Time not found, so assumed to be unbanned.");
                // We have now checked that the user is not banned, that the time is top 200 worthy, that the score doesn't exist in the db, but is banned.
                post_sp_pb(entry.profilenumber.clone(), entry.score, wr, id, timestamp, current_rank, &map_json);
            },
        }
    }
}

pub fn post_sp_pb(profilenumber: String, score: i32, wr: i32, id: i32, timestamp: NaiveDateTime, current_rank: Option<i32>, map_json: &Vec<SpRanked>) -> bool{
    let mut wr_gain = 0;
    if score >= wr{
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
    match pb_vec{
        Some(pb_vec) => {
            let current_pb = pb_vec
                .into_iter()
                .nth(0)
                .unwrap();
            previous_id = Some(current_pb.score);
        },
        None => (),
    }

    let mut post_rank: Option<i32> = None;
    for entry in map_json.iter(){
        if entry.map_data.score == score{
            // They have the same rank
            post_rank = Some(entry.rank)
        } else if entry.map_data.score > score{
            // They will temporarily have the same rank, as when the board re-calculates, the values for the other maps will change. But this value only tracks the inital rank at time of update.
            post_rank = Some(entry.rank)
        }
    }

    let new_score = Changelog {
        time_gained: Some(timestamp),
        profile_number: profilenumber, 
        score: score,
        map_id: id.to_string(),
        wr_gain: wr_gain,
        previous_id: previous_id, // id of last PB
        post_rank: post_rank, // New rank as of this score update
        pre_rank: current_rank, // Rank prior to this score update
        has_demo: Some(0),
        banned: 0,
        submission: 0,
        youtube_id: None,
        coopid: None,
        note: None,
        category: Some("any%".to_string()),
        id: 0,
    };
    // TODO: POST UPDATE TO API ENDPOINT
    false
}

/// Version of `filter_entries` for coop, using different logic.
pub fn filter_entries_coop(id: i32, start: i32, end: i32, timestamp: NaiveDateTime, banned_users: Vec<String>, data: &XmlTag<Vec<Entry>>){
    let url = format!("http://localhost:8080/api/maps/coop/{id}", id = id);
    let map_json: Vec<CoopRanked> = reqwest::blocking::get(&url)
        .expect("Error in query to our local API (Make sure the webserver is running")
        .json()
        .expect("Error in converting our API values to JSON");

    let mut existing_hash: HashMap<String, i32> = HashMap::with_capacity(400);
    let worst_score = map_json[199].map_data.score;
    let mut new_times = Vec::new();
    // We attempt to insert both players into the hashmap. This way we get all players with a top 200 in coop.
    for rank in map_json.iter(){
        existing_hash.insert(rank.map_data.profile_number1.clone(), rank.map_data.score);
        existing_hash.insert(rank.map_data.profile_number2.clone(), rank.map_data.score);
    }

    // Filter out all scores that exist
    for entry in data.value.iter(){
        match existing_hash.get(&entry.steam_id.value){
            Some(score) => { // If they're in top 200
                if score > &entry.score.value{ // If the score is better than their pre-existing score
                    match check_cheated(&entry.steam_id.value, &banned_users){
                        false => new_times.push(entry.clone()), // Add to list of new coop times to be processed.
                        _ => (), // Banned user, ignore time.
                    }
                }
            } _ => {
                if entry.score.value > worst_score{ // If they're not in top 200, but they should be
                    match check_cheated(&entry.steam_id.value, &banned_users){
                        false => new_times.push(entry.clone()), // Add to list of new coop times to be processed.
                        _ => (),
                    }
                }
            }
        }
    }
    // We now have a list of all the times we want to add to the db. Now we attempt to match them.
    // We need to reference a list of all banned times on a map.
    let ban_url = format!("http://localhost:8080/api/maps/coop/banned/{id}", id = id);
    let banned_users: Vec<CoopBanned> = reqwest::blocking::get(&ban_url)
        .expect("Error in query to our local API (Make sure the webserver is running")
        .json()
        .expect("Error in converting our API values to JSON");
    
    // Check to see if any of the times are banned on our leaderboards

    // The times that aren't banned should be parsed to see if there are matching times
    // If the times are matching, all old times are filtered, and no banned times are taken into consideration, 
    // it's fair to assume the times were gotten together between two people
    
    // Create individual changelog entries, and create a bundled coop time to represent the new times

    // Push to the database.
}

pub fn check_cheated(id: &String, banned_users: &Vec<String>) -> bool{
    for entry in banned_users.iter(){
        if entry == id{
            return true;
        }
    }
    false
}