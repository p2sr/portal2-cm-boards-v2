#![allow(dead_code)]
use super::exporting::*;
use super::fetching_coop::*;
use super::fetching_sp::*;
use crate::models::{
    Entry, FetchingData, GetPlayerSummariesWrapper, Leaderboards, PostSP, SpBanned, SpRanked,
    Users, XmlTag,
};
use anyhow::Result;
use log::{debug, trace};
use serde_xml_rs::from_reader;
use std::collections::HashMap;

/// Grabs the map at the current ID from valve's API and caches times.
pub fn fetch_entries(data: FetchingData) -> Result<Leaderboards> {
    let url = format!(
        "https://steamcommunity.com/stats/Portal2/leaderboards/{}?xml=1&start={}&end={}",
        data.id, data.start, data.end
    );
    let text = reqwest::blocking::get(&url)?.text()?;
    let leaderboard: Leaderboards = from_reader(text.as_bytes()).expect("XML Error in parsing");

    // Print to cache
    if !cache_leaderboard(data.id, text) {
        trace!("The cache is unchanged for map {}", data.id);
        return Ok(leaderboard); // Return early, our cache is unchanged.
    }
    match data.is_coop {
        false => filter_entries_sp(data, &leaderboard.entries)?,
        true => filter_entries_coop(data, &leaderboard.entries)?,
    }
    Ok(leaderboard)
}

/// Breaking apart the modules that filted out the list to times that aren't banned/cheated.
pub fn validate_entries(
    data: &XmlTag<Vec<Entry>>,
    existing_hash: HashMap<&str, (i32, i32)>,
    banned_users: &[String],
    id: i32,
    worst_score: i32,
) -> (HashMap<String, i32>, Vec<SpBanned>) {
    let mut current_rank: HashMap<String, i32> = HashMap::new();
    let mut not_cheated: Vec<SpBanned> = Vec::new();
    // TODO: Potentially turn this into a macro? This basic shape is reused.
    for entry in data.value.iter() {
        match existing_hash.get(&entry.steam_id.value as &str) {
            Some((score, rank)) => {
                // The user has a time in top X scores currently
                if score > &entry.score.value {
                    debug!(
                        "New better time for user {} on map_id {}",
                        entry.steam_id.value, id
                    ); // Add to leaderboards.
                    current_rank.insert(entry.steam_id.value.clone(), *rank);
                    if !banned_users.contains(&entry.steam_id.value) {
                        not_cheated.push(SpBanned {
                            profile_number: entry.steam_id.value.clone(),
                            score: entry.score.value,
                        })
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
                    if !banned_users.contains(&entry.steam_id.value) {
                        not_cheated.push(SpBanned {
                            profile_number: entry.steam_id.value.clone(),
                            score: entry.score.value,
                        })
                    }
                }
            }
        }
    }
    (current_rank, not_cheated)
}

pub fn check_user(profile_number: &str) -> Result<Users> {
    let url = format!("http://localhost:8080/api/v1/user/{}", profile_number);
    Ok(reqwest::blocking::get(&url)?.json::<Users>()?)
}

#[allow(dead_code)]
pub fn update_image(profile_number: &str) -> Result<String> {
    let api_key = dotenv::var("STEAM_API_KEY").expect("Cannot find STEAM_API_KEY in ./.env");

    let steam_api_url = format!(
        "https://api.steampowered.com/ISteamUser/GetPlayerSummaries/v2/?key={}&steamids={}",
        api_key, profile_number
    );
    match reqwest::blocking::get(&steam_api_url)?.json::<GetPlayerSummariesWrapper>() {
        Ok(user) => Ok(user.response.players[0].avatarfull.clone()),
        Err(e) => {
            eprintln!("Error getting response from steam API -> {}", e);
            // Default image
            Ok("http://media.steampowered.com/steamcommunity/public/images/avatars/f9/f91787b7fb6d4a2cb8dee079ab457839b33a8845_full.jpg".to_string())
        }
    }
}
// TODO:
// 620 - Portal 2.
// http://api.steampowered.com/IPlayerService/GetOwnedGames/v0001/?key={}&steamid={}}&format=json
#[allow(dead_code)]
pub fn add_user(profile_number: String) -> Result<Users> {
    // http://steamcommunity.com/profiles/{}/?xml=1
    // GET https://api.steampowered.com/ISteamUser/GetPlayerSummaries/v2/
    let api_key = dotenv::var("STEAM_API_KEY").expect("Cannot find STEAM_API_KEY in ./.env");

    let steam_api_url = format!(
        "https://api.steampowered.com/ISteamUser/GetPlayerSummaries/v2/?key={}&steamids={}",
        api_key, profile_number
    );
    let user = reqwest::blocking::get(&steam_api_url)?.json::<GetPlayerSummariesWrapper>()?;

    let new_user = Users {
        profile_number,
        board_name: None,
        steam_name: Some(user.response.players[0].personaname.clone()),
        banned: false,
        registered: 0,
        avatar: Some(user.response.players[0].avatarfull.clone()),
        ..Default::default()
    };

    let url = String::from("http://localhost:8080/api/v1/user");
    let client = reqwest::blocking::Client::new();
    Ok(client.post(&url).json(&new_user).send()?.json::<Users>()?)
}
