use crate::models::{ChangelogInsert, PostSP, SpPbHistory};
use anyhow::Result;
use log::trace;
use std::cmp::Ordering;

/// Upload sp PB to the database
pub fn post_sp_pb(params: PostSP) -> Result<()> {
    let new_score = construct_sp_score(params)?;
    let client = reqwest::blocking::Client::new();
    let post_url = "http://localhost:8080/api/v1/sp/post_score".to_string();
    let _ = client
        .post(&post_url)
        .json(&new_score)
        .send()?
        .json::<i64>()?;
    Ok(())
}

/// Constructs a ChangelogInsert from the parameters given to post to the web server.
pub fn construct_sp_score(params: PostSP) -> Result<ChangelogInsert> {
    // Grab the PB history.
    let url = format!(
        "http://localhost:8080/api/v1/sp/history?map_id={}&profile_number={}",
        params.id, params.profile_number
    );
    let pb_history = match reqwest::blocking::get(&url)?.json::<SpPbHistory>() {
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

    let previous_id: Option<i32>;
    let past_score: Option<i32>;
    let pb_vec = pb_history.pb_history;
    match pb_vec {
        Some(pb_vec) => {
            let current_pb = pb_vec.into_iter().next();
            if let Some(s) = current_pb {
                let current_pb = s;
                previous_id = Some(current_pb.id as i32);
                past_score = Some(current_pb.score);
            } else {
                previous_id = None;
                past_score = None;
            }
        }
        None => {
            past_score = None;
            previous_id = None;
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
    let prerank: Option<i32> = match params.current_rank.get(&params.profile_number) {
        Some(rank) => Some(*rank),
        None => None,
    };
    let mut score_delta: Option<i32> = None;
    if let Some(i) = past_score {
        score_delta = Some(params.score - i);
    }
    Ok(ChangelogInsert {
        timestamp: Some(params.timestamp),
        profile_number: params.profile_number.to_string(),
        score: params.score,
        map_id: params.id.to_string(),
        demo_id: None,
        banned: false,
        youtube_id: None,
        previous_id, // id of last PB
        coop_id: None,
        post_rank,         // New rank as of this score update
        pre_rank: prerank, // Rank prior to this score update
        submission: 0,
        note: None,
        category_id: params.cat_id,
        score_delta,
        verified: None,
        admin_note: None,
    })
}
