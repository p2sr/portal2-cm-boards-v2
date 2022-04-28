#![allow(dead_code)]
use super::super::FetchingData;
use super::exporting::*;
use super::uploading_coop::*;
use super::uploading_sp::*;
use crate::models::datamodels::{
    CoopDataUtil, CoopRanked, Entry, GetPlayerSummariesWrapper, Leaderboards, PostCoop, PostSP,
    SpBanned, SpRanked, Users, XmlTag,
};
use crate::LIMIT_MULT_COOP;
use crate::LIMIT_MULT_SP;
use anyhow::Result;
use chrono::prelude::*;
use log::{debug, trace};
use rayon::prelude::*;
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
    banned_users: Vec<String>,
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

/// Handles comparison with the current leaderboards to see if any user has a new best time
pub fn filter_entries_sp(data: FetchingData, lb: &XmlTag<Vec<Entry>>) -> Result<()> {
    let url = format!("http://localhost:8080/api/v1/map/sp/{id}", id = data.id);
    let map_json: Vec<SpRanked> = reqwest::blocking::get(&url)?.json()?;
    let mut existing_hash: HashMap<&str, (i32, i32)> =
        HashMap::with_capacity((data.end / LIMIT_MULT_SP) as usize);

    let worst_score = map_json[map_json.len() - 1].map_data.score;
    let wr = map_json[0].map_data.score;

    for rank in map_json.iter() {
        existing_hash.insert(
            &rank.map_data.profile_number,
            (rank.map_data.score, rank.rank),
        );
    }

    // TODO: Implement a per-map threshold???
    let (current_rank, not_cheated) =
        validate_entries(lb, existing_hash, data.banned_users, data.id, worst_score);
    // We grab the list of banned times from our API.
    // Filter out any times that are banned from the list of potential runs.
    // The list of new scores is probably relatively low, it would be easier to just send the score information to an endpoint and have it check.
    // Working around the borrow checker because these values are all Copy.
    let id = data.id;
    let timestamp = data.timestamp;
    let cat_id = data.cat_id;
    let _: Vec<()> = not_cheated
        .into_par_iter()
        .map(|entry| {
            // TODO: Remove this unwrap
            check_existing_banned(id, entry, timestamp, &current_rank, &map_json, cat_id).unwrap()
        })
        .collect();
    Ok(())
}

pub fn check_existing_banned(
    id: i32,
    entry: SpBanned,
    timestamp: NaiveDateTime,
    current_rank: &HashMap<String, i32>,
    map_json: &[SpRanked],
    cat_id: i32,
) -> Result<()> {
    let ban_url = format!(
        "http://localhost:8080/api/v1/sp/banned/{}?profile_number={}&score={}",
        id, entry.profile_number, entry.score
    );
    match reqwest::blocking::get(&ban_url)?.json()? {
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
            post_sp_pb(PostSP {
                profile_number: entry.profile_number.clone(),
                score: entry.score,
                id,
                timestamp,
                current_rank: &current_rank,
                map_json: &map_json,
                cat_id,
            })?;
        }
    }
    Ok(())
}

/// Version of `filter_entries` for coop, using different logic.
pub fn filter_entries_coop(data: FetchingData, lb: &XmlTag<Vec<Entry>>) -> Result<()> {
    let url = format!("http://localhost:8080/api/v1/map/coop/{id}", id = data.id);
    let map_json: Vec<CoopRanked> = reqwest::blocking::get(&url)?.json()?;
    let mut existing_hash: HashMap<&str, (i32, i32)> =
        HashMap::with_capacity(((data.end / LIMIT_MULT_COOP) * 2) as usize);
    let worst_score = map_json[map_json.len() - 1].map_data.score;
    // let wr = map_json[0].map_data.score;
    // We attempt to insert both players into the hashmap. This way we get all players with a top X score in coop.
    for rank in map_json.iter() {
        existing_hash.insert(
            &rank.map_data.profile_number1,
            (rank.map_data.score, rank.rank),
        );
        existing_hash.insert(
            &rank.map_data.profile_number2,
            (rank.map_data.score, rank.rank),
        );
    }

    let (current_rank, not_banned_players) =
        validate_entries(lb, existing_hash, data.banned_users, data.id, worst_score);

    // Check to see if any of the times are banned on our leaderboards
    // Do an SP style check here first, as we want to ensure none of the times are old and banned.
    // The issue with doing this step pre-bundled would be if long-standing, banned times are bundled before checking
    // to see that they're old, banned times on the leaderboard, our assumption about all scores being new and together
    // falls apart.
    let mut not_cheated = Vec::new(); // Becomes the vector of times that are not from banned players, and do not exist in the changelog.
    for entry in not_banned_players.into_iter() {
        let ban_url = format!(
            "http://localhost:8080/api/v1/coop/banned/{}?profile_number={}&score={}",
            data.id, entry.profile_number, entry.score
        );
        match reqwest::blocking::get(&ban_url)?.json::<bool>()? {
            true => debug!("The time was found, so the time is banned. Ignore"),
            false => not_cheated.push(entry),
        }
    }

    // The times that aren't banned should be parsed to see if there are matching times
    // If the times are matching, all old times are filtered, and no banned times are taken into consideration,
    // it's fair to assume the times were gotten together between two people
    let mut already_bundled: HashMap<String, i32> = HashMap::new();
    // Contains the bundled entries (if profile_number2 is None, there is no mathcing time)
    // TODO: Can we hold references here? Or do lifetimes bite us too hard.
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
        post_coop_pb(PostCoop {
            profile_number1: entry.profile_number1.clone(),
            profile_number2: entry.profile_number2.clone(),
            score: entry.score,
            id: data.id,
            timestamp: data.timestamp,
            current_rank: &current_rank,
            map_json: &map_json,
            cat_id: data.cat_id,
        })?;
    }
    Ok(())
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
