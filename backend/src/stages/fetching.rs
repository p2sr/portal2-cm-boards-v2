use std::collections::HashMap;
use serde_xml_rs::from_reader;
use super::exporting::*;
use chrono::prelude::*;

use crate::models::datamodels::{SPMap, SPRanked, SpBanned, CoopMap, CoopRanked, Leaderboards, XmlTag, Entry};

/// Grabs the map at the current ID from valve's API and caches times.
pub fn fetch_entries(id: i32, start: i32, end: i32, timestamp: DateTime<Utc>, is_coop: bool) -> Leaderboards {
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

/// Handles comparison with the current leaderboards to see if any user has a new best time
pub fn filter_entries_sp(id: i32, start: i32, end: i32, timestamp: DateTime<Utc>, banned_users: Vec<String>, data: &XmlTag<Vec<Entry>>){
    let url = format!("http://localhost:8080/api/maps/sp/{id}", id = id);
    let map_json: Vec<SPRanked> = reqwest::blocking::get(&url)
        .expect("Error in query to our local API (Make sure the webserver is running")
        .json()
        .expect("Error in converting our API values to JSON");
    let mut existing_hash: HashMap<String, i32> = HashMap::with_capacity(200);
    let worst_score = map_json[199].map_data.score;
    
    #[allow(clippy::redundant_pattern_matching)]
    for rank in map_json.iter(){
        if let Some(_) = existing_hash.insert(rank.map_data.profile_number.clone(), rank.map_data.score) {}
    } 
    // TODO: Implement a per-map threshold???
    // Potentially turn this into a macro? This basic shape is reused.
    for entry in data.value.iter(){
        match existing_hash.get(&entry.steam_id.value){
            // The user has a time in top 200 currently
            Some(score) => {
                if score > &entry.score.value{
                    // Add to leaderboards.
                    println!("New better time for user {} on map_id {}", entry.steam_id.value, id);
                    match check_cheated(&entry.steam_id.value, &banned_users) {
                        false => println!("User not banned"), // Send score to be added to db. 
                        _ => (),
                    }
                }
            } _ => { // The user is not currently in top 200.
                if entry.score.value > worst_score{
                    println!("User {} is new to top 200 on {}, we need to add their time!", entry.steam_id.value, id);
                    match check_cheated(&entry.steam_id.value, &banned_users) {
                        false => println!("User not banned"), // Send score to be added to db. 
                        _ => (),
                    }
                }
            }
        }
    }
    // We grab the list of banned times from our API.
    let ban_url = format!("http://localhost:8080/api/maps/sp/banned/{id}", id = id);
    let banned_users: Vec<SpBanned> = reqwest::blocking::get(&ban_url)
        .expect("Error in query to our local API (Make sure the webserver is running")
        .json()
        .expect("Error in converting our API values to JSON");
    
    // Filter out any times that are banned from the list of potential runs.
    
    // Create a changelog entry for the time.
    
}
/// Version of `filter_entries` for coop, using different logic.
pub fn filter_entries_coop(id: i32, start: i32, end: i32, timestamp: DateTime<Utc>, banned_users: Vec<String>, data: &XmlTag<Vec<Entry>>){
    let url = format!("http://localhost:8080/api/maps/coop/{id}", id = id);
    let map_json: Vec<CoopRanked> = reqwest::blocking::get(&url)
        .expect("Error in query to our local API (Make sure the webserver is running")
        .json()
        .expect("Error in converting our API values to JSON");
    // What the fuck do we do here?

    let mut existing_hash: HashMap<String, i32> = HashMap::with_capacity(400);
    let worst_score = map_json[199].map_data.score;
    let mut new_times = Vec::new();
    // We attempt to insert both players into the hashmap. This way we get all players with a top 200 in coop.
    for rank in map_json.iter(){
        if let Some(_) = existing_hash.insert(rank.map_data.profile_number1.clone(), rank.map_data.score) {}
        if let Some(_) = existing_hash.insert(rank.map_data.profile_number2.clone(), rank.map_data.score) {}
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
    // let banned_users: Vec<CoopBanned> = reqwest::blocking::get(&ban_url)
    //     .expect("Error in query to our local API (Make sure the webserver is running")
    //     .json()
    //     .expect("Error in converting our API values to JSON");
    
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