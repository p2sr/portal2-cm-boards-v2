use std::collections::HashMap;
use serde_xml_rs::from_reader;
use super::exporting::*;
use chrono::prelude::*;

use crate::models::datamodels::SPMap;
use crate::models::datamodels::SPRanked;
//use crate::models::*;

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
//  Result<Leaderboards, Box<dyn std::error::Error>>
pub fn fetch_entries(id: i32, start: i32, end: i32, timestamp: DateTime<Utc>) -> Leaderboards {
    let url = format!(
        "https://steamcommunity.com/stats/{game}/leaderboards/{id}?xml=1&start={start}&end={end}",
        game = "Portal2",
        id = id,
        start = start,
        end = end
    );
    // Please don't be mad at unwraps... 
    // TODO: Better error handling
    let text = reqwest::blocking::get(&url).expect("Error in request to valve API").text().expect("Error in writing the result from Valve's API to text");
    //Print to cache
    let is_updated = cache_leaderboard(id, text.clone());
    if is_updated{
        println!("The cache is updated for map {}", id);
    }
    let leaderboard: Leaderboards = from_reader(text.as_bytes()).expect("XML Error in parsing");
    // println!("fetched {} entries on {} ({}-{})",
    //     leaderboard.result_count.value, id, start, end
    // );
    filter_entries(id, start, end, timestamp, &leaderboard.entries);
    leaderboard
    // Filter out leaderboard entries that don't need to be added?
}

/// `filter_entries` handles comparison with the current leaderboards to see if any user has a new best time
pub fn filter_entries(id: i32, start: i32, end: i32, timestamp: DateTime<Utc>, data: &XmlTag<Vec<Entry>>){
    let url = format!("http://localhost:8080/api/maps/sp/{id}", id = id);
    let map_json: Vec<SPRanked> = reqwest::blocking::get(&url).expect("Error in query to our local API").json().expect("Error in converting our API values to JSON");
    let mut existing_hash: HashMap<String, i32> = HashMap::with_capacity(200);
    let worst_score = map_json[199].map_data.score;
    
    #[allow(clippy::redundant_pattern_matching)]
    for rank in map_json.iter(){
        if let Some(_) = existing_hash.insert(rank.map_data.profile_number.clone(), rank.map_data.score) {}
        // match existing_hash.insert(rank.map_data.profile_number.clone(), rank.map_data.score){
        //     Some(_) => (),
        //     _ => ()
        // }
    }
    
    // TODO: Filter out cheaters / cheated times.
    // TODO: Implement a per-map threshold???
    for entry in data.value.iter(){
        match existing_hash.get(&entry.steam_id.value){
            // If the current user we're checking has a new, better time, we need to add this entry to the leadboards
            Some(score) => {
                if score > &entry.score.value{
                    // Add to leaderboards.
                    //println!("New better time for user {} on map_id {}", entry.steam_id.value, id);
                }
            }
            _ => {
                if entry.score.value > worst_score{
                    //println!("This time is too bad to be added : {} vs {}", entry.score.value, worst_score);
                }
            }
        }
    }
}