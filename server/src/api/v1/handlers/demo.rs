use crate::controllers::models::{Changelog, ChangelogInsert, DemoInsert, Demos, Maps};
use crate::tools::config::Config;
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
// TODO: Assume that this will be used to upload a demo to an existing changelog entry.
#[post("/demo")]
pub async fn receive_multiparts(
    mut payload: Multipart,
    config: web::Data<Config>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let mut file_id: Option<String> = None;
    let mut values = DemoInsert::default();
    //println!("{} - {} - {}", config.backblaze.keyid, config.backblaze.key, config.backblaze.bucket);
    while let Ok(Some(mut field)) = payload.try_next().await {
        // Note: content_disposition() now returns a &ContentDisposition, rather than an Option<ContentDisposition>
        let mut content_data = Vec::new();
        while let Some(Ok(chunk)) = field.next().await {
            content_data.extend(chunk);
        }
        let field_name = field
            .content_disposition()
            .get_name()
            .unwrap_or("NO-KEY-PROVIDED");
        let file_name = field.content_disposition().get_filename();
        // Handle the case where we were passed a file
        if let Some(file_name) = file_name {
            use std::fs;
            match fs::create_dir_all("./demos") {
                Ok(_) => (),
                Err(e) => {
                    return HttpResponse::InternalServerError()
                        .body(format!("Failed to create demo directory locally -> {}", e))
                }
            }
            let file = OpenOptions::new()
                .create(true)
                .write(true)
                .open(format!("./demos/{}", file_name));
            match file {
                Ok(mut res) => match res.write_all(&content_data) {
                    Ok(_) => (),
                    Err(e) => {
                        return HttpResponse::InternalServerError()
                            .body(format!("Failed to write demo locally -> {}", e))
                    }
                },
                Err(e) => {
                    return HttpResponse::InternalServerError()
                        .body(format!("Failed to write demo locally -> {}", e))
                }
            };
            // TODO: Parse Demo
            file_id = match upload_demo(&config, file_name).await {
                Ok(fid) => fid,
                Err(e) => {
                    eprintln!("Error with File upload -> {:?}", e);
                    None
                }
            };
            // Delete Demo
            let res = remove_file(format!("./demos/{}", file_name));
            match res {
                Ok(_) => (),
                Err(e) => {
                    return HttpResponse::InternalServerError()
                        .body(format!("Failed to delete demo locally -> {}", e))
                }
            }
        } else {
            // Handle the case where we are passed a text value.
            let result_string = match str::from_utf8(&content_data) {
                Ok(our_string) => our_string,
                Err(e) => {
                    eprintln!("Invalid UTF-8 sequence: {}", e);
                    "ERROR"
                }
            };
            match field_name {
                "partner_name" => values.partner_name = Some(result_string.to_string()),
                "parsed_successfully" => {
                    values.parsed_successfully = {
                        match result_string {
                            "false" => false,
                            "true" => true,
                            _ => false,
                        }
                    }
                }
                "sar_version" => values.sar_version = Some(result_string.to_string()),
                "cl_id" => values.cl_id = result_string.parse::<i64>().unwrap_or(0),
                _ => eprintln!("Got an unexpected field."),
            }
        }
    }
    if let Some(file_id) = file_id {
        values.file_id = file_id;
    }
    //println!("{:#?}", values);
    let res = Demos::insert_demo(&pool, values).await;
    match res {
        Ok(id) => HttpResponse::Ok().json(id),
        Err(e) => HttpResponse::InternalServerError()
            .body(format!("Failed to add demo to database -> {}", e)),
    }
}

/// Accepts field values for both a changelog, and a demo file.
/// Expects the following fields:
///      * "timestamp"        -> String: %Y-%m-%d %H:%M:%S
///      * "profile_number"   -> String: Steam ID #
///      * "score"            -> i32:    Current board time format         // TODO: Update how scores are handled.
///      * "map_id"           -> String: Steam ID for the map
///        "youtube_id"       -> String: Youtube URL Extension.            // NOTE: ONLY PASS IF THERE IS A YOUTUBE ID
///        "note"             -> String: Note for the run
///        "category_id"      -> i32:    ID for the category being played  // TODO: Fill out with default
/// *Fields marked with * are required
#[post("/demos/changelog")]
pub async fn changelog_with_demo(
    mut payload: Multipart,
    config: web::Data<Config>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let mut file_name = String::default();
    let mut changelog_insert = ChangelogInsert {
        submission: true,
        ..Default::default()
    };
    let mut demo_insert = DemoInsert::default();
    while let Ok(Some(mut field)) = payload.try_next().await {
        let mut content_data = Vec::new();
        while let Some(Ok(chunk)) = field.next().await {
            content_data.extend(chunk);
        }
        let field_name = field
            .content_disposition()
            .get_name()
            .unwrap_or("NO-KEY-PROVIDED");
        let fname = field.content_disposition().get_filename();

        if let Some(fname) = fname {
            use std::fs;
            match fs::create_dir_all("./demos") {
                Ok(_) => (),
                Err(e) => {
                    return HttpResponse::InternalServerError()
                        .body(format!("Failed to create demo directory locally -> {}", e))
                }
            }
            let file = OpenOptions::new()
                .create(true)
                .write(true)
                .open(format!("./demos/{}", fname));
            match file {
                Ok(mut res) => match res.write_all(&content_data) {
                    Ok(_) => (),
                    Err(e) => {
                        return HttpResponse::InternalServerError()
                            .body(format!("Failed to write demo locally -> {}", e))
                    }
                },
                Err(e) => {
                    return HttpResponse::InternalServerError()
                        .body(format!("Failed to write demo locally -> {}", e))
                }
            };
            file_name = fname.to_string();
            // TODO: Parse Demo
        } else {
            // Handle the case where we are passed a text value.
            let result_string = str::from_utf8(&content_data).unwrap_or("ERROR");
            match field_name {
                "timestamp" => {
                    changelog_insert.timestamp =
                        match NaiveDateTime::parse_from_str(result_string, "%Y-%m-%d %H:%M:%S") {
                            Ok(val) => Some(val),
                            Err(_) => None,
                        }
                }
                "profile_number" => changelog_insert.profile_number = result_string.to_string(),
                "score" => {
                    changelog_insert.score = match result_string.parse::<i32>() {
                        Ok(val) => val,
                        Err(_) => {
                            return HttpResponse::BadRequest()
                                .body("Invalid score, could not parse")
                        }
                    }
                }
                "map_id" => changelog_insert.map_id = result_string.to_string(),
                "youtube_id" => changelog_insert.youtube_id = Some(result_string.to_string()),
                "note" => changelog_insert.note = Some(result_string.to_string()),
                "category_id" => {
                    changelog_insert.category_id = match result_string.parse::<i32>() {
                        Ok(val) => val,
                        Err(_) => {
                            return HttpResponse::BadRequest()
                                .body("Invalid category_id, could not parse")
                        }
                    }
                }
                _ => (),
            }
            // Make sure these are not defaults before we use them for calculating score information
            if changelog_insert.score != 0
                && !changelog_insert.profile_number.is_empty()
                && !changelog_insert.map_id.is_empty()
            {
                use super::sp::check_for_valid_score;
                let res = check_for_valid_score(
                    pool.get_ref(),
                    changelog_insert.profile_number.clone(),
                    changelog_insert.score,
                    changelog_insert.map_id.clone(),
                    config.proof.results,
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
                            // USER IS BANNED, DO NOT ADD A TIME FOR THEM
                            return HttpResponse::NotFound().body("User is banned");
                        }
                    }
                    Err(e) => {
                        eprintln!("Error finding newscore details -> {:#?}", e);
                        // Cannot find user.
                    }
                }
            }
        }
    }
    // Make sure we got our required data.
    if changelog_insert.score == 0
        || !changelog_insert.profile_number.is_empty()
        || !changelog_insert.map_id.is_empty()
        || changelog_insert.timestamp == None
    {
        eprintln!("Invalid information provided\n{:#?}", changelog_insert);
        return HttpResponse::BadRequest().body("Incorrect information passed.");
    }
    // If the cateogry wasn't set, get the default
    if changelog_insert.category_id == 0 {
        let default_category =
            Maps::get_default_cat(pool.get_ref(), changelog_insert.map_id.clone()).await;
        changelog_insert.category_id = match default_category {
            Ok(Some(id)) => id,
            _ => {
                eprintln!("Error getting default category, map name is most likely incorrect.");
                return HttpResponse::BadRequest()
                    .body("Default category not found for map_id given, assumed incorrect map_id");
            }
        }
    }
    // Add Changelog entry to database.
    let res = Changelog::insert_changelog(pool.get_ref(), changelog_insert).await;
    demo_insert.cl_id = match res {
        Ok(id) => id,
        Err(e) => {
            eprintln!("Error adding score to database -> {:?}", e);
            return HttpResponse::InternalServerError().body("Error adding new score to database.");
        }
    };
    // TODO: How do we want demo files named?
    let file_id = match upload_demo(&config, &file_name).await {
        Ok(fid) => fid,
        Err(e) => {
            eprintln!("Error with File upload -> {:?}", e);
            None
        }
    };
    // Delete Demo
    let res = remove_file(format!("./demos/{}", file_name));
    match res {
        Ok(_) => (),
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Failed to delete demo locally -> {}", e))
        }
    }
    if let Some(file_id) = file_id {
        demo_insert.file_id = file_id;
    }
    // Add demo entry to database.
    let res = Demos::insert_demo(pool.get_ref(), demo_insert).await;
    match res {
        Ok(id) => HttpResponse::Ok().json(id),
        Err(e) => HttpResponse::InternalServerError()
            .body(format!("Failed to add demo to database -> {}", e)),
    }
}

async fn upload_demo(config: &web::Data<Config>, file_name: &str) -> Result<Option<String>> {
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
