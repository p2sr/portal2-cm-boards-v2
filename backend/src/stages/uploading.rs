use super::exporting::*;
use chrono::prelude::*;
use log::{debug, error, trace};
use serde_xml_rs::from_reader;
use std::collections::HashMap;

use crate::models::datamodels::{
    ChangelogInsert, CoopBundled, CoopBundledInsert, CoopDataUtil, CoopMap, CoopRanked, Entry,
    Leaderboards, SpBanned, SpMap, SpPbHistory, SpRanked, XmlTag,
};

/// Upload sp PB to the database
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
        "http://localhost:8080/api/v1/map/sp/{}/{}",
        id, profile_number
    );
    let res = reqwest::blocking::get(&url)
        .expect("Error in query to our local API (Make sure the webserver is running")
        .json::<SpPbHistory>();
    let pb_history = match res {
        // TODO: Handle this with new user creation.
        Ok(s) => s,
        Err(e) => {
            trace!("{}", e);
            SpPbHistory {
                user_name: None,
                avatar: None,
                pb_history: None,
            }
        }
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
        score_delta = Some(score - i);
    }
    let mut cat_id = 0;
    let url = format!(
        "http://localhost:8080/api/v1/category/default_category/{}",
        id
    );
    let res = reqwest::blocking::get(&url)
        .expect("Error in query to our local API (Make sure the webserver is running")
        .json::<i32>();
    let pb_history = match res {
        Ok(s) => cat_id = s,
        Err(e) => {
            trace!("{}", e);
        }
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
        post_rank: post_rank, // New rank as of this score update
        pre_rank: prerank,    // Rank prior to this score update
        submission: false,
        note: None,
        category_id: cat_id,
        score_delta: score_delta,
        verified: None,
        admin_note: None,
    };
    let client = reqwest::blocking::Client::new();
    let post_url = "http://localhost:8080/api/v1/sp/post_score".to_string();
    let res = client
        .post(&post_url)
        .json(&new_score)
        .send()
        .expect("Error querying our local API")
        .json::<i64>();
    match res {
        Ok(s) => {
            trace!("{}", s)
        }
        Err(e) => {
            error!("{}", e);
            return false;
        }
    }
    true
}

/// Upload coop PB to database
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
            "http://localhost:8080/api/v1/map/sp/{}/{}",
            id, profile_number1
        ); // TODO: Handle crashing if no PB history is found.
        let pb_history1: SpPbHistory = reqwest::blocking::get(&url)
            .expect("Error in query to our local API (Make sure the webserver is running")
            .json()
            .expect("Error in converting our API values to JSON");
        let url = format!(
            "http://localhost:8080/api/v1/map/sp/{}/{}",
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
            score_delta1 = Some(score - i);
        }
        let mut score_delta2: Option<i32> = None;
        if let Some(i) = past_score {
            score_delta2 = Some(score - i);
        }
        let mut cat_id = 0;
        let url = format!(
            "http://localhost:8080/api/v1/category/default_category/{}",
            id
        );
        let res = reqwest::blocking::get(&url)
            .expect("Error in query to our local API (Make sure the webserver is running")
            .json::<i32>();
        let pb_history = match res {
            Ok(s) => cat_id = s,
            Err(e) => {
                trace!("{}", e);
            }
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
            post_rank: post_rank, // New rank as of this score update
            pre_rank: prerank1,   // Rank prior to this score update
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
            post_rank: post_rank, // New rank as of this score update
            pre_rank: prerank2,   // Rank prior to this score update
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
        let post_url = "http://localhost:8080/api/v1/sp/post_score".to_string();
        let res = client
            .post(&post_url)
            .json(&score1)
            .send()
            .expect("Error querying our local API")
            .json::<i64>();
        match res {
            Ok(s) => {
                new_id1 = s;
            }
            Err(e) => {
                error!("{}", e)
            }
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
            }
            Err(e) => {
                error!("{}", e)
            }
        }
        let bundle = CoopBundledInsert {
            p_id1: profile_number1,
            p_id2: Some(profile_number2),
            p1_is_host: None,
            cl_id1: new_id1,
            cl_id2: Some(new_id2),
        };
        let post_url = "http://localhost:8080/api/v1/coop/post_score".to_string();
        let res = client
            .post(&post_url)
            .json(&bundle)
            .send()
            .expect("Error querying our local API")
            .json::<i64>();
        match res {
            Ok(s) => {
                trace!("{}", s);
            }
            Err(e) => {
                debug!("{}", e);
            }
        }
    }

    true
}
