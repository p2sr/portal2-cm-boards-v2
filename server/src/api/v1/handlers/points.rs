use actix_web::{get, post, web, HttpResponse, Responder};
use anyhow::{Result, Error};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::io::Read;

/// Writes out json data to cache points for the boards.
pub async fn write_to_file(id: &str, data: web::Json<PointsWrapper>) -> Result<(), Error> {
    let path_str = format!("./points/{}.json", id.to_string());
    let path = Path::new(&path_str);
    serde_json::to_writer(&File::create(path)?, &data)
        .map(|_| ())
        .map_err(|err| err.into())
}

// TODO: Should be able to make this faster, but stuck with weird limitations to mapping result values from serde.
/// Reads in json from the cache for the passed in ID.
pub async fn read_from_file(id: &str) -> Result<PointsWrapper, Error> {
    let path_str = format!("./points/{}.json", id.to_string());
    let path = Path::new(&path_str);
    let mut file = File::open(path)?;
    let mut buff = String::new();
    file.read_to_string(&mut buff)?;
    let res: PointsWrapper = serde_json::from_str(&buff)?;
    Ok(res)
}

/// Wrapper struct that contains an optional ID to identify chapter (None if not a chapter), and a hashmap that contains all point information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointsWrapper {
    id: Option<i32>,
    points: HashMap<String, Points>,
}

/// Point information for a given player. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Points{
    points: f32,
    score: i32, // TODO: Need to change the format to support SAR timing
    num_scores: i32,
    total_rank_sum: i32,
    worst: (i32, String), 
    best: (i32, String),
}

/// Update single player points data.
#[post("/points/sp")]
async fn post_points_sp(data: web::Json<PointsWrapper>) -> impl Responder {
    // Cache data in .json files
    match write_to_file("sp", data).await {
        Ok(_) => HttpResponse::Ok().body("Success"), // TODO: Fix error handling (return values?)
        _ => HttpResponse::NotFound().body("Error updaing score entries for sp"),
    }
}

/// Gget single player points data.
#[get("points/sp")]
async fn get_points_sp() -> impl Responder {
    let res = read_from_file("sp").await;
    match res {
        Ok(sp_points) => HttpResponse::Ok().json(sp_points),
        _ => HttpResponse::NotFound().body("No score entries for SP found."),
    }
}

/// Update coop points data.
#[post("/points/coop")]
async fn post_points_coop(data: web::Json<PointsWrapper>) -> impl Responder {
    // Cache data in .json files
    match write_to_file("coop", data).await {
        Ok(_) => HttpResponse::Ok().body("Success"), // TODO: Fix error handling (return values?)
        _ => HttpResponse::NotFound().body("Error updaing score entries for coop"),
    }
}

/// Get coop points data.
#[get("points/coop")]
async fn get_points_coop() -> impl Responder {
    let res = read_from_file("coop").await;
    match res {
        Ok(coop_points) => HttpResponse::Ok().json(coop_points),
        _ => HttpResponse::NotFound().body("No score entries found."),
    }
}

/// Update chapter data, uses JSON ID (see [PointsWrapper]).
#[post("/points/chapter")]
async fn post_points_chapter(data: web::Json<PointsWrapper>) -> impl Responder {
    match write_to_file(&data.id.expect("No chapter ID for chapter").to_string(), data).await {
        Ok(_) => HttpResponse::Ok().body("Success"), // TODO: Fix error handling (return values?)
        _ => HttpResponse::NotFound().body("Error updaing score entries for chapter"),
    }
}

/// Get points data for a specific chapter.
#[get("points/chapter/{id}")]
async fn get_points_chapter(id: web::Path<u64>) -> impl Responder {
    let res = read_from_file(&id.to_string()).await;
    match res {
        Ok(chapter_points) => HttpResponse::Ok().json(chapter_points),
        _ => HttpResponse::NotFound().body("No coop score entries found."),
    }
}

/// Update overall points data.
#[post("/points/overall")]
async fn post_points_overall(data: web::Json<PointsWrapper>) -> impl Responder {
    match write_to_file("overall", data).await {
        Ok(_) => HttpResponse::Ok().body("Success"), // TODO: Fix error handling (return values?)
        _ => HttpResponse::NotFound().body("Error updaing score entries for overall"),
    }
}

/// Get overall points data.
#[get("points/overall")]
async fn get_points_overall() -> impl Responder {
    let res = read_from_file("overall").await;
    match res {
        Ok(overall_points) => HttpResponse::Ok().json(overall_points),
        _ => HttpResponse::NotFound().body("No score entries found."),
    }
}
