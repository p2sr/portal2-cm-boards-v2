use anyhow::{bail, Result};
use num::pow;
use sqlx::PgPool;
use std::collections::{HashMap, HashSet};

use crate::models::changelog::{CalcValues, Changelog, ChangelogInsert, SubmissionChangelog};
use crate::models::coop::{CoopMap, CoopRanked};
use crate::models::maps::Maps;
use crate::models::sp::SpMap;
use crate::models::users::Users;

use super::cache::CacheState;
use super::config::Config;

pub type Transaction<'a> = sqlx::Transaction<'a, sqlx::Postgres>;

/// Calcultes the score using the pre-existing iVerb point formula.
#[inline(always)]
pub fn score(i: i32) -> f32 {
    if i > 200 {
        0.0
    } else {
        let i = i as f32;
        let res: f32 = pow(200.0 - (i - 1.0), 2) / 200.0;
        if 1.0 > res {
            1.0
        } else {
            res
        }
    }
}

/// Grabs the default category IDs for all maps as a HashMap.
pub async fn get_default_cat_ids(pool: &PgPool) -> HashMap<String, i32> {
    Maps::get_all_default_cats(pool).await.unwrap()
}

/// Filters out all obsolete times from the result, then truncates to x entries.
pub async fn filter_coop_entries(coop_entries: Vec<CoopMap>, limit: usize) -> Vec<CoopRanked> {
    let mut coop_entries_filtered = Vec::new();
    let mut remove_dups: HashSet<String> = HashSet::with_capacity(limit);
    remove_dups.insert("N/A".to_string());
    let mut i = 1;
    for entry in coop_entries.into_iter() {
        match remove_dups.insert(entry.profile_number1.clone()) {
            // If player 1 has a better time, check to see if player 2 doesn't.
            false => match remove_dups.insert(entry.profile_number2.clone()) {
                false => (),
                true => {
                    coop_entries_filtered.push(CoopRanked {
                        map_data: entry.clone(),
                        rank: i,
                        points: score(i),
                    });
                    i += 1;
                }
            },
            // This case handles if player 1 doesn't have a better time, and it tries to add player 2 in as well, if two has a better time or not, this is included.
            true => match remove_dups.insert(entry.profile_number2.clone()) {
                false => {
                    coop_entries_filtered.push(CoopRanked {
                        map_data: entry.clone(),
                        rank: i,
                        points: score(i),
                    });
                    i += 1;
                }
                true => {
                    coop_entries_filtered.push(CoopRanked {
                        map_data: entry.clone(),
                        rank: i,
                        points: score(i),
                    });
                    i += 1;
                }
            },
        }
    }
    coop_entries_filtered.truncate(limit);
    coop_entries_filtered
}

/// Checks if a score is valid, if it is, returns post_rank, pre_rank, score_delta, previous_id
pub async fn check_for_valid_score(
    pool: &PgPool,
    cl: &SubmissionChangelog,
    limit: i32,
) -> Result<CalcValues> {
    let mut values = CalcValues::default();
    match Users::check_banned(pool, &cl.profile_number).await {
        Ok(b) => {
            if b {
                values.banned = true;
                return Ok(values);
            }
        }
        Err(e) => {
            // Assuming someone is manually/automatically submitting a demo, a user account should be created for them on the boards.
            // TODO: Maybe this changes when AUTH changes?
            eprintln!("User assumed not found -> {:?}", e);
            bail!("User does not exist");
        }
    }
    let cl_res = Changelog::get_sp_pb_history(
        pool,
        &cl.profile_number,
        &cl.map_id,
        cl.category_id.unwrap(),
        cl.game_id.unwrap_or(1),
    )
    .await;
    let cl_res = match cl_res {
        Ok(x) => {
            if x.is_empty() {
                return Ok(values);
            } else {
                x
            }
        }
        Err(e) => {
            eprintln!("Error with sp pb history -> {:?}", e);
            eprintln!("Assume there is not sp_pb_histroy for the player.");
            return Ok(values);
        }
    };

    if cl_res[0].score <= cl.score {
        bail!("Current score is the same, or better.")
    }
    values.score_delta = Some(cl_res[0].score - cl.score);
    values.previous_id = Some(cl_res[0].id);
    // Assuming there is a PB History, there must be other scores, this should return a valid list of ranked maps.
    let cl_ranked = SpMap::get_sp_map_page(
        pool,
        &cl.map_id,
        limit,
        cl.category_id.unwrap(),
        cl.game_id.unwrap_or(1),
    )
    .await
    .unwrap();
    for (i, entry) in cl_ranked.iter().enumerate() {
        if entry.score >= cl.score {
            values.post_rank = Some(i as i32 + 1);
        }
        if entry.profile_number == cl.profile_number {
            values.pre_rank = Some(i as i32 + 1);
        }
    }
    Ok(values)
}

/// Returns a ChangelogInsert that should be valid to insert.
///
/// Checks for a past score on the map for the user.
///
/// Score is invalid if any of the following are true
/// 1. The user is banned.
/// 2. The user has a time on the same map, with the same score (time).
/// 3. The user does not exist (and cannot be added from Steam).
///
/// This function handles the error case where the user is valid on steam, but does not currently exist in our database.
pub async fn get_valid_changelog_insert(
    pool: &PgPool,
    config: &Config,
    cache: &CacheState,
    mut cl: SubmissionChangelog,
) -> Result<ChangelogInsert> {
    if cl.category_id.is_none() {
        cl.category_id = Some(cache.default_cat_ids[&cl.map_id]);
    } // Steps 1 & 2
    let values = match check_for_valid_score(pool, &cl, config.proof.results).await {
        Ok(details) => {
            if details.banned {
                bail!("User is banned");
            } else {
                details
            }
        }
        Err(e) => {
            // Step 3
            eprintln!("Error checking valid score details -> {e}");
            // Try to insert the user into the users table.
            match Users::new_from_steam(&config.steam.api_key, &cl.profile_number).await {
                Ok(user) => match Users::insert_new_users(pool, user).await {
                    Ok(_) => CalcValues::default(),
                    _ => bail!("Could not add new user to database."),
                },
                Err(e) => {
                    eprintln!("Could not get user from steam -> {e}");
                    bail!("Invalid user steam_id provided.");
                }
            }
        }
    };
    // Step 4
    Ok(ChangelogInsert::new_from_submission(cl, values, &cache.default_cat_ids).await)
}
