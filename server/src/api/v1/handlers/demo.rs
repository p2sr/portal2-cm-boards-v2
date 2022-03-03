use crate::controllers::models::{
    Changelog, ChangelogInsert, DemoInsert, Demos, SubmissionChangelog,
};
use crate::tools::cache::CacheState;
use crate::tools::config::Config;
use crate::tools::helpers::check_for_valid_score;
use actix_multipart::Multipart;
use actix_web::{post, web, HttpResponse, Responder};
use anyhow::Result;
use chrono::NaiveDateTime;
use futures::{StreamExt, TryStreamExt};
use raze::api::*;
use raze::utils::*;
use sqlx::PgPool;
use std::fs::remove_file;
use std::fs::OpenOptions;
use std::io::Write;
use std::str;

//  a. Handle renaming/db interactions (update demo table/specific time that is being uploaded)
//  b. Pass to backblaze
//  c. Look to see if there is anything special needed for auto-submit
//  d. Integrate Parsing
// Code Reference: https://github.com/Ujang360/actix-multipart-demo/blob/main/src/main.rs
// TODO: Allow for sar version or partner name?
/// Accepts field values for both a changelog, and a demo file.
/// ## Expects the following fields:
///
/// **Required Parameters**: timestamp, profile_number, score, map_id
///
/// **Optional Parameters**: youtube_id, note, cat_id
///
/// ## Parameters:
///
/// - **timestamp**    
///     - `String`: `%Y-%m-%d %H:%M:%S` (use `%20` to denote a space)
/// - **profile_number**
///     - `String`: Steam ID Number
/// - **score**         
///     - `i32`: Current board time format         
/// - **map_id**       
///     - `String`: Steam ID for the map
/// - **youtube_id**
///     - `String`: Youtube URL Extension.
/// - **note**          
///     - `String`: Note for the run
/// - **category_id**   
///     - `i32`: ID for the category being played  
///
/// ## Example endpoints:       
/// - `/api/v1/demos/changelog?timestamp=2020-08-18%2024:60:60&profile_number=76561198040982247&score=1763&map_id=47763`
///
#[post("/demos/changelog")]
pub async fn changelog_with_demo(
    mut payload: Multipart,
    config: web::Data<Config>,
    query: web::Query<SubmissionChangelog>,
    cache: web::Data<CacheState>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    // This function heavily utilizes helper functions to make error propagation easier, and reduce the # of match arms
    let mut file_name = String::default();
    let query = query.into_inner();
    let mut changelog_insert =
        ChangelogInsert::new_from_submission(query, cache.into_inner().default_cat_ids.clone())
            .await;
    let res = check_for_valid_score(
        pool.get_ref(),
        changelog_insert.profile_number.clone(),
        changelog_insert.score,
        changelog_insert.map_id.clone(),
        config.proof.results,
        changelog_insert.category_id,
    )
    .await;

    match res {
        Ok(details) => {
            if !details.banned {
                changelog_insert.previous_id = details.previous_id;
                changelog_insert.post_rank = details.post_rank;
                changelog_insert.pre_rank = details.pre_rank;
                changelog_insert.score_delta = details.score_delta;
            } else {
                eprintln!("USER IS BANNED, DO NOT ADD A TIME FOR THEM");
                return HttpResponse::NotFound().body("User is banned");
            }
        }
        Err(e) => {
            eprintln!("Error finding newscore details -> {:#?}", e);
            return HttpResponse::NotFound().body("User not found, or better time exists.");
        }
    }
    match parse_and_write_multipart(&mut payload, &mut file_name).await {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error parsing or writing the file. -> {}", e);
            return HttpResponse::BadRequest().body("Error parsing or write the file.");
        }
    }
    // Add Changelog/Demo entries to database.
    match add_to_database(pool.get_ref(), changelog_insert, &config, &file_name, true).await {
        Ok((cl_id, demo_id)) => HttpResponse::Ok().json((cl_id, demo_id)),
        Err(e) => {
            eprintln!("Error with adding changelog/demo insert -> {}", e);
            HttpResponse::InternalServerError()
                .body("Failed updating demo/changelog entries to database.")
        }
    }
}

/// Adds a demo and changelog insert to the database
/// The debug value passed will remove the added changelog/demo entries inserted, and skip uploading the file for quicker debugging.
async fn add_to_database(
    pool: &PgPool,
    changelog_insert: ChangelogInsert,
    config: &Config,
    file_name: &str,
    debug: bool,
) -> Result<(i64, i64)> {
    let mut demo_insert = DemoInsert::default();
    let cl_id = Changelog::insert_changelog(pool, changelog_insert).await?;
    demo_insert.cl_id = cl_id;
    // TODO: How do we want demo files named?
    let file_id = if !debug {
        upload_demo(config, &file_name).await?
    } else {
        Some(format!("{}.dem", file_name))
    };
    // Delete Demo
    remove_file(format!("./demos/{}", file_name))?;
    if let Some(file_id) = file_id {
        demo_insert.file_id = file_id;
    }
    // Add demo entry to database.
    let demo_id = Demos::insert_demo(pool, demo_insert).await?;
    // Update changelog to have the new demo_id
    Changelog::update_demo_id_in_changelog(pool, cl_id, demo_id).await?;
    if debug {
        Changelog::delete_changelog(pool, cl_id).await?;
        Demos::delete_demo(pool, demo_id).await?;
    }
    Ok((cl_id, demo_id))
}

/// Helper function that handles parsing the multipart and writing the file out locally
async fn parse_and_write_multipart(payload: &mut Multipart, file_name: &mut String) -> Result<()> {
    while let Ok(Some(mut field)) = payload.try_next().await {
        let mut content_data = Vec::new();
        while let Some(Ok(chunk)) = field.next().await {
            content_data.extend(chunk);
        }
        let fname = field.content_disposition().get_filename();

        if let Some(fname) = fname {
            use std::fs;
            fs::create_dir_all("./demos")?;
            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .open(format!("./demos/{}", fname))?;
            file.write_all(&content_data)?;
            *file_name = fname.to_string();
            // TODO: Parse Demo
        }
    }
    Ok(())
}

/// Handles uploading the demo file
async fn upload_demo(config: &Config, file_name: &str) -> Result<Option<String>> {
    let client = reqwest::ClientBuilder::new().build().unwrap();
    // Ref: https://docs.rs/raze/0.4.1/raze/api/fn.b2_authorize_account.html
    let auth = b2_authorize_account(
        &client,
        format!("{}:{}", config.backblaze.keyid, config.backblaze.key),
    )
    .await
    .unwrap();
    let upload_auth = b2_get_upload_url(&client, &auth, config.backblaze.bucket.clone())
        .await
        .unwrap();
    let file = tokio::fs::File::open(format!("./demos/{}", file_name))
        .await
        .unwrap();
    let metadata = file.metadata().await.unwrap();
    let size = metadata.len();
    let modf = metadata
        .modified()
        .unwrap()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        * 1000;

    let param = FileParameters {
        file_path: file_name,
        file_size: size,
        content_type: None,
        content_sha1: Sha1Variant::HexAtEnd,
        last_modified_millis: modf,
    };

    let stream = reader_to_stream(file);
    let stream = BytesStreamHashAtEnd::wrap(stream);
    let stream = BytesStreamThrottled::wrap(stream, 500000000);

    let body = reqwest::Body::wrap_stream(stream);
    let resp1 = b2_upload_file(&client, &upload_auth, body, param)
        .await
        .unwrap();
    Ok(resp1.file_id)
}
