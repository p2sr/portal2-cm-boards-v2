use super::fetching::validate_entries;
use super::uploading_sp::post_sp_pb;
use crate::models::{Entry, FetchingData, PostSP, SpBanned, SpRanked, XmlTag};
use crate::LIMIT_MULT_SP;
use anyhow::Result;
use chrono::prelude::*;
use log::trace;
use rayon::prelude::*;
use std::collections::HashMap;

/// Handles comparison with the current leaderboards to see if any user has a new best time
pub fn filter_entries_sp(data: FetchingData, lb: &XmlTag<Vec<Entry>>) -> Result<()> {
    let url = format!("http://localhost:8080/api/v1/map/sp/{id}", id = data.id);
    let map_json: Vec<SpRanked> = reqwest::blocking::get(&url)?.json()?;
    let mut existing_hash: HashMap<&str, (i32, i32)> =
        HashMap::with_capacity((data.end / LIMIT_MULT_SP) as usize);

    let worst_score = map_json[map_json.len() - 1].map_data.score;
    // let wr = map_json[0].map_data.score;

    for rank in map_json.iter() {
        existing_hash.insert(
            &rank.map_data.profile_number,
            (rank.map_data.score, rank.rank),
        );
    }

    // TODO: Implement a per-map threshold???
    let (current_rank, not_cheated) =
        validate_entries(lb, existing_hash, &data.banned_users, data.id, worst_score);
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
                current_rank,
                map_json,
                cat_id,
            })?;
        }
    }
    Ok(())
}
