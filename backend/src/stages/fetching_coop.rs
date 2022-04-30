use super::fetching::validate_entries;
use super::uploading_coop::post_coop_pb;
use crate::models::{CoopDataUtil, CoopRanked, Entry, FetchingData, PostCoop, SpBanned, XmlTag};
use crate::LIMIT_MULT_COOP;
use anyhow::Result;
use rayon::prelude::*;
use std::collections::HashMap;

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
        validate_entries(lb, existing_hash, &data.banned_users, data.id, worst_score);

    // Check to see if any of the times are banned on our leaderboards
    // Do an SP style check here first, as we want to ensure none of the times are old and banned.
    // The issue with doing this step pre-bundled would be if long-standing, banned times are bundled before checking
    // to see that they're old, banned times on the leaderboard, our assumption about all scores being new and together
    // falls apart.
    // TODO: This option work-around feels so damn stupid.
    let not_cheated: Vec<Result<Option<SpBanned>>> = not_banned_players
        .into_par_iter()
        .map(|entry| {
            let ban_url = format!(
                "http://localhost:8080/api/v1/sp/banned/{}?profile_number={}&score={}",
                data.id, entry.profile_number, entry.score
            );
            match reqwest::blocking::get(&ban_url)?.json::<bool>()? {
                true => Ok(None),
                false => Ok(Some(entry)),
            }
        })
        .collect();
    // The times that aren't banned should be parsed to see if there are matching times
    // If the times are matching, all old times are filtered, and no banned times are taken into consideration,
    // it's fair to assume the times were gotten together between two people
    let mut already_bundled: HashMap<String, i32> = HashMap::new();
    // Contains the bundled entries (if profile_number2 is None, there is no mathcing time)
    // TODO: Can we hold references here? Or do lifetimes bite us too hard.
    let mut bundled_entries = Vec::new();
    for entry in not_cheated.iter().flatten().flatten() {
        for entry2 in not_cheated.iter().flatten().flatten() {
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
    let _: Vec<()> = bundled_entries
        .into_par_iter()
        .map(|entry| {
            // TODO: Better error handling for a failure to insert new coop score.
            post_coop_pb(PostCoop {
                profile_number1: entry.profile_number1.clone(),
                profile_number2: entry.profile_number2.clone(),
                score: entry.score,
                id: data.id,
                timestamp: data.timestamp,
                current_rank: &current_rank,
                map_json: &map_json,
                cat_id: data.cat_id,
            })
            .unwrap();
        })
        .collect();
    Ok(())
}
