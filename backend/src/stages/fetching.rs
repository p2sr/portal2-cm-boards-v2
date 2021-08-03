use std::collections::HashMap;
use serde_xml_rs::from_reader;
use super::exporting::*;
use chrono::prelude::*;

use crate::models::datamodels::{SPMap, SPRanked, CoopMap, CoopRanked, Leaderboards, XmlTag, Entry};

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
}
/// Version of `filter_entries` for coop, using different logic.
pub fn filter_entries_coop(id: i32, start: i32, end: i32, timestamp: DateTime<Utc>, banned_users: Vec<String>, data: &XmlTag<Vec<Entry>>){
    let url = format!("http://localhost:8080/api/maps/coop/{id}", id = id);
    let map_json: Vec<CoopRanked> = reqwest::blocking::get(&url)
        .expect("Error in query to our local API (Make sure the webserver is running")
        .json()
        .expect("Error in converting our API values to JSON");
}

pub fn check_cheated(id: &String, banned_users: &Vec<String>) -> bool{
    for entry in banned_users.iter(){
        if entry == id{
            return true;
        }
    }
    false
}