use crate::models::datamodels::{ChangelogInsert, CoopBundledInsert, PostCoop, SpPbHistory};
use anyhow::Result;
use log::debug;
use std::cmp::Ordering;

/// Upload coop PB to database
pub fn post_coop_pb(params: PostCoop) -> Result<()> {
    let profile_number1 = params.profile_number1.clone();
    let profile_number2 = params.profile_number2.clone();

    let (score1, score2) = construct_coop_score(params)?;
    // Insert both changelog entries, retrieve their IDs, create bundle
    let client = reqwest::blocking::Client::new();
    let post_url = "http://localhost:8080/api/v1/sp/post_score".to_string();
    let new_id1 = client.post(&post_url).json(&score1).send()?.json::<i64>()?;
    let new_id2 = client.post(&post_url).json(&score2).send()?.json::<i64>()?;
    let bundle = CoopBundledInsert {
        p_id1: profile_number1,
        p_id2: profile_number2,
        p1_is_host: None,
        cl_id1: new_id1,
        cl_id2: Some(new_id2),
    };
    let post_url = "http://localhost:8080/api/v1/coop/post_score".to_string();
    let _res = client.post(&post_url).json(&bundle).send()?.json::<i64>()?;
    Ok(())
}

pub fn construct_coop_score(params: PostCoop) -> Result<(ChangelogInsert, ChangelogInsert)> {
    // Handle there being a partner
    if let Some(profile_number2) = params.profile_number2 {
        let previous_id1: Option<i32>;
        let previous_id2: Option<i32>;
        let past_score1: Option<i32>;
        let past_score2: Option<i32>;
        // Grab the PB history. For now, we're just going to use 2 calls to our API rather than a combined call. (We'll use SP here).
        let url = format!(
            "http://localhost:8080/api/v1/sp/history?map_id={}&profile_number={}",
            params.id, params.profile_number1
        );
        let pb_history1: SpPbHistory = reqwest::blocking::get(&url)?.json()?;
        let url = format!(
            "http://localhost:8080/api/v1/sp/history?map_id={}&profile_number={}",
            params.id, profile_number2
        );
        let pb_history2: SpPbHistory = reqwest::blocking::get(&url)?.json()?;
        match pb_history1.pb_history {
            Some(pb_vec) => {
                let current_pb = pb_vec.into_iter().next();
                if let Some(s) = current_pb {
                    let current_pb = s;
                    previous_id1 = Some(current_pb.id as i32);
                    past_score1 = Some(current_pb.score);
                } else {
                    previous_id1 = None;
                    past_score1 = None;
                }
            }
            None => {
                previous_id1 = None;
                past_score1 = None;
            }
        }
        match pb_history2.pb_history {
            Some(pb_vec) => {
                let current_pb = pb_vec.into_iter().next();
                if let Some(s) = current_pb {
                    let current_pb = s;
                    previous_id2 = Some(current_pb.id as i32);
                    past_score2 = Some(current_pb.score);
                } else {
                    previous_id2 = None;
                    past_score2 = None;
                }
            }
            None => {
                previous_id2 = None;
                past_score2 = None;
            }
        }

        let mut post_rank: Option<i32> = None;
        for entry in params.map_json.iter() {
            match entry.map_data.score.cmp(&params.score) {
                Ordering::Equal => post_rank = Some(entry.rank),
                Ordering::Greater => post_rank = Some(entry.rank),
                _ => (),
            }
        }
        #[allow(clippy::manual_map)]
        let prerank1: Option<i32> = match params.current_rank.get(&params.profile_number1) {
            Some(rank) => Some(*rank),
            None => None,
        };
        #[allow(clippy::manual_map)]
        let prerank2: Option<i32> = match params.current_rank.get(&profile_number2) {
            Some(rank) => Some(*rank),
            None => None,
        };

        let mut score_delta1: Option<i32> = None;
        if let Some(i) = past_score1 {
            score_delta1 = Some(params.score - i);
        }
        let mut score_delta2: Option<i32> = None;
        if let Some(i) = past_score2 {
            score_delta2 = Some(params.score - i);
        }
        //println!("{}", cat_id);

        // We first need to upload individually as changelog entries, get the result from that insert (the changelogID it creates, then use that for the bundling process).
        let score1 = ChangelogInsert {
            timestamp: Some(params.timestamp),
            profile_number: params.profile_number1,
            score: params.score,
            map_id: params.id.to_string(),
            demo_id: None,
            banned: false,
            youtube_id: None,
            previous_id: previous_id1, // id of last PB
            coop_id: None,
            post_rank,          // New rank as of this score update
            pre_rank: prerank1, // Rank prior to this score update
            submission: false,
            note: None,
            category_id: params.cat_id,
            score_delta: score_delta1,
            verified: None,
            admin_note: None,
        };

        let score2 = ChangelogInsert {
            timestamp: Some(params.timestamp),
            profile_number: profile_number2,
            score: params.score,
            map_id: params.id.to_string(),
            demo_id: None,
            banned: false,
            youtube_id: None,
            previous_id: previous_id2, // id of last PB
            coop_id: None,
            post_rank,          // New rank as of this score update
            pre_rank: prerank2, // Rank prior to this score update
            submission: false,
            note: None,
            category_id: params.cat_id,
            score_delta: score_delta2,
            verified: None,
            admin_note: None,
        };
        debug!("{:#?}", score1);
        debug!("{:#?}", score2);
        Ok((score1, score2))
    } else {
        unimplemented!();
    }
}
