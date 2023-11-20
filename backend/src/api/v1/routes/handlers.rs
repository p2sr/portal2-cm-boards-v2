use crate::models::*;
use crate::points::calc_points;
use crate::stages::fetching::fetch_entries;
use crate::stages::uploading::upload_changelog_and_demo;
use crate::stages::uploading::upload_new_pfp;
use crate::stages::uploading_coop::upload_coop_bundled;
use crate::{LIMIT_MULT_COOP, LIMIT_MULT_SP, OFFICIAL_COOP, OFFICIAL_SP};
use actix_web::{get, web, HttpResponse, Responder};
use anyhow::Result;
use chrono::prelude::*;
use rayon::prelude::*;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[get("/recalculate_points/{id}")]
pub async fn rcp(id: web::Path<i32>) -> impl Responder {
    // Assume that a negative `id` means that the user wants to recalculate all points.
    web::block(move || {
        let id = id.into_inner();
        if id < 0 {
            calc_points(None);
        } else {
            // Get a list of maps altered? In this case it would probably just be the ID.
            calc_points(Some(vec![id]));
        }
    })
    .await
    .unwrap();
    "Points calculation successful."
}

// TODO: It might be much faster to just grab all the data from the web server at the start.
// Given how long it takes to go through and query for each of the like, thousands of potential times.
#[get("/fetch_all")]
pub async fn fetch_all(
    limit: web::Data<i32>,
    cat_ids: web::Data<HashMap<String, i32>>,
) -> impl Responder {
    web::block(move || {
        let banned_users: Vec<String> =
            reqwest::blocking::get("http://localhost:8080/api/v1/banned_users_all")
                .expect("Error in query to our local API (Make sure the webserver is running")
                .json()
                .expect("Error in converting our API values to JSON");
        let limit = limit.into_inner();
        let timestamp = Utc::now().naive_utc();
        let _res_sp: Vec<_> = OFFICIAL_SP
            .into_par_iter()
            .map(|map_id| {
                fetch_entries(FetchingData {
                    id: *map_id,
                    start: 0,
                    end: *limit * LIMIT_MULT_SP,
                    timestamp,
                    banned_users: banned_users.clone(),
                    is_coop: false,
                    cat_id: cat_ids[&map_id.to_string()],
                })
            })
            .collect();
        let _res_cp: Vec<_> = OFFICIAL_COOP
            .into_par_iter()
            .map(|map_id| {
                fetch_entries(FetchingData {
                    id: *map_id,
                    start: 0,
                    end: *limit * LIMIT_MULT_COOP,
                    timestamp,
                    banned_users: banned_users.clone(),
                    is_coop: true,
                    cat_id: cat_ids[&map_id.to_string()],
                })
            })
            .collect();
    })
    .await
    .unwrap();
    "Success fetching all scores."
}

// TODO: It looks like currently, there is a conflict between the async thread pool, and the threadpool for rayon, as this is now VERY slow.
// TODO: Seems to currenty upload without caring about new user? Post new user information to `/user`.
#[get("/fetch_sp/{map_id}")]
pub async fn fetch_sp(
    map_id: web::Path<i32>,
    limit: web::Data<i32>,
    cat_ids: web::Data<HashMap<String, i32>>,
) -> impl Responder {
    let banned_users: Vec<String> = reqwest::get("http://localhost:8080/api/v1/banned_users_all")
        .await
        .expect("Error in query to our local API (Make sure the webserver is running")
        .json()
        .await
        .expect("Error in converting our API values to JSON");
    web::block(move || {
        let limit = limit.into_inner();
        let timestamp = Utc::now().naive_utc();
        let map_id = map_id.into_inner();
        let _ = fetch_entries(FetchingData {
            id: map_id,
            start: 0,
            end: *limit * LIMIT_MULT_SP,
            timestamp,
            banned_users,
            is_coop: false,
            cat_id: cat_ids[&map_id.to_string()],
        });
    })
    .await
    .unwrap();
    "Success fetching sp map."
}

#[get("/fetch_coop/{map_id}")]
pub async fn fetch_coop(
    map_id: web::Path<i32>,
    limit: web::Data<i32>,
    cat_ids: web::Data<HashMap<String, i32>>,
) -> impl Responder {
    let banned_users: Vec<String> = reqwest::get("http://localhost:8080/api/v1/banned_users_all")
        .await
        .expect("Error in query to our local API (Make sure the webserver is running")
        .json()
        .await
        .expect("Error in converting our API values to JSON");
    web::block(move || {
        let limit = limit.into_inner();
        let timestamp = Utc::now().naive_utc();
        let map_id = map_id.into_inner();
        let _ = fetch_entries(FetchingData {
            id: map_id,
            start: 0,
            end: *limit * LIMIT_MULT_COOP,
            timestamp,
            banned_users,
            is_coop: true,
            cat_id: cat_ids[&map_id.to_string()],
        });
    })
    .await
    .unwrap();
    "Success fetching coop map."
}

#[get("/fetch_pfp/{profile_number}")]
pub async fn fetch_pfp(profile_number: web::Path<String>) -> impl Responder {
    let profile_number = profile_number.into_inner();
    let upload = web::block(move || upload_new_pfp(&profile_number))
        .await
        .unwrap();
    match upload {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(e) => {
            eprintln!("Error updating avatar -> {}", e);
            HttpResponse::NotModified().body("Could not update avatar.")
        }
    }
}

pub async fn read_timestamp() -> Result<NaiveDate> {
    let path = Path::new("./src/recent_check.txt");
    let mut file = File::open(path)?;
    let mut buff = String::new();
    file.read_to_string(&mut buff)?;
    let res: NaiveDate = serde_json::from_str(&buff)?;
    Ok(res)
}

pub async fn write_timestamp(timestamp: NaiveDate) -> Result<()> {
    let path = Path::new("./src/recent_check.txt");
    serde_json::to_writer(&File::create(path)?, &timestamp)
        .map(|_| ())
        .map_err(|err| err.into())
}

#[get("/update_from_legacy_boards")]
pub async fn update_from_legacy_boards(cat_ids: web::Data<HashMap<String, i32>>) -> impl Responder {
    let prev_timestamp = match read_timestamp().await {
        Ok(ts) => ts,
        Err(_e) => NaiveDate::parse_from_str("2022-05-12", "%Y-%m-%d").unwrap(),
    };
    let timestamp = Utc::now().naive_utc(); // Write this to file.

    // let path = Path::new("recent_check.txt");
    // serde_json::to_writer(&File::create(path).unwrap(), &timestamp).unwrap();
    let url = format!("https://board.portal2.sr/changelog/json?boardName=&profileNumber=&chapter=&chamber=&startDate={}&endDate=&startRank=&endRank=&sp=1&coop=1&wr=&demo=&yt=&submission=&pending=", prev_timestamp);
    let entries = reqwest::get(&url)
        .await
        .unwrap()
        .json::<Vec<LegacyBoardEntry>>()
        .await
        .unwrap();
    let mut entry_buffer: VecDeque<(ChangelogInsert, Option<DemoInsert>)> = VecDeque::new();
    // Going in reverse order means that we should be able to update the previous ID correctly for scores that have been updated multiple times.
    for entry in entries.into_iter().rev() {
        let cat_id = cat_ids[&entry.mapid];
        let is_coop = entry.chapterId.parse::<i32>().map(|x| x <= 6).unwrap();
        if let Some((cl, demo)) = entry.convert_to_changelog(cat_id).await {
            if is_coop {
                entry_buffer.push_back((cl, demo)); // Handle the coop cases after we've gone through all entries.
            } else if let Some(d) = demo {
                // Add the changelog entry, then add the demo insert, then update the changelog entry.
                match crate::stages::uploading::upload_changelog_and_demo(&cl, &d).await {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!("{e}");
                        panic!();
                    }
                }
            } else {
                // SP no demo
                let new_id = cl_no_demo(&cl).await.unwrap();
                println!("Successfully uploaded changelog entry without a demo {new_id}");
            }
        }
    }
    // Handle bundling of coop times.
    while !entry_buffer.is_empty() {
        let x = entry_buffer.pop_front().unwrap();
        let mut pos: Option<usize> = None;
        for (i, entry) in entry_buffer.iter().enumerate() {
            if entry.0.profile_number != x.0.profile_number
                && entry.0.score == x.0.score
                && entry.0.map_id == x.0.map_id
            {
                pos = Some(i);
            }
        }
        if let Some(j) = pos {
            create_coop_bundled(vec![x, entry_buffer.remove(j).unwrap()])
                .await
                .unwrap();
        } else {
        }
    }
    // Rewrite our file.
    write_timestamp(NaiveDate::from_ymd(
        timestamp.year(),
        timestamp.month(),
        timestamp.day(),
    ))
    .await
    .unwrap();
    HttpResponse::Ok().body("Ok")
}

pub async fn create_coop_bundled(v: Vec<(ChangelogInsert, Option<DemoInsert>)>) -> Result<()> {
    let mut ids: Vec<i64> = Vec::new();
    for entry in v.iter() {
        if let Some(d) = &entry.1 {
            // Add the changelog entry, then add the demo insert, then update the changelog entry.
            ids.push(upload_changelog_and_demo(&entry.0, d).await.unwrap());
        } else {
            // No Demo
            let new_id = cl_no_demo(&entry.0).await.unwrap();
            println!("Successfully uploaded changelog entry without a demo {new_id}");
            ids.push(new_id);
        }
    }
    match ids.len() {
        1 => {
            // Create a coop bundled with one score.
            match upload_coop_bundled(
                CoopBundledInsert::create_from_single(
                    v[0].0.profile_number.clone(),
                    ids[0],
                    &v[0].0.map_id,
                )
                .await,
            )
            .await
            {
                Ok(id) => {
                    update_changelog_with_coop_id(ids[0], id).await.unwrap();
                    // Update the changelog entry.
                    println!("Successfully uploaded coop bundled with single id: {id}")
                }
                Err(e) => eprintln!("Could not upload single coop bundled. -> {e}"),
            }
        }
        2 => {
            // Create a coop_bundled with both scores.
            match upload_coop_bundled(CoopBundledInsert {
                p_id1: v[0].0.profile_number.clone(),
                p_id2: Some(v[1].0.profile_number.clone()),
                p1_is_host: None,
                cl_id1: ids[0],
                cl_id2: Some(ids[1]),
            })
            .await
            {
                Ok(id) => {
                    update_changelog_with_coop_id(ids[0], id).await.unwrap();
                    update_changelog_with_coop_id(ids[1], id).await.unwrap();
                    // Update both the changelog entries.
                    println!("Successfully uploaded coop bundled with two ids: {id}")
                }
                Err(e) => eprintln!("Could not upload coop bundled. -> {e}"),
            }
        }
        _ => unreachable!(),
    }
    Ok(())
}

pub async fn update_changelog_with_coop_id(cl_id: i64, coop_id: i64) -> Result<()> {
    let url = format!("http://localhost:8080/api/v1/coop/update_changelog/{cl_id}/{coop_id}");
    let _ = reqwest::Client::new()
        .put(&url)
        .send()
        .await?
        .json::<Changelog>()
        .await?;
    Ok(())
}

pub async fn cl_no_demo(cl: &ChangelogInsert) -> Result<i64> {
    Ok(reqwest::Client::new()
        .post("http://localhost:8080/api/v1/sp/post_score")
        .json(cl)
        .send()
        .await?
        .json::<i64>()
        .await?)
}
