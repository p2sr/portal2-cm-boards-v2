use actix_web::{get, post, web, HttpResponse, Responder};
use anyhow::{Result, Error};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::io::Read;


pub async fn write_to_file(id: &str, data: web::Json<PointsWrapper>) -> Result<(), Error> {
    let path_str = format!("./points/{}.json", id.to_string());
    let path = Path::new(&path_str);
    serde_json::to_writer(&File::create(path)?, &data)
        .map(|_| ())
        .map_err(|err| err.into())
}

pub async fn read_from_file(file_name: &str) -> Result<Vec<PointsWrapper>, Error> {
    let path_str = format!("./points/{}.json", file_name.to_string());
    let path = Path::new(&path_str);
    
    let mut file = File::open(path)?;
    let mut buff = String::new();
    file.read_to_string(&mut buff)?;
    let pw: Vec<PointsWrapper> = serde_json::from_str(&buff)?;
    Ok(pw)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointsWrapper {
    id: Option<i32>,
    points: HashMap<String, Points>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Points{
    points: f32,
    score: i32, // TODO: Need to change the format to support SAR timing
    num_scores: i32,
    total_rank_sum: i32,
    worst: (i32, String), 
    best: (i32, String),
}

#[post("/points/sp")]
async fn post_points_sp(data: web::Json<PointsWrapper>) -> impl Responder {
    // Cache data in .json files
    match write_to_file("sp", data).await {
        Ok(_) => HttpResponse::Ok().body("Success"), // TODO: Fix error handling (return values?)
        _ => HttpResponse::NotFound().body("Error updaing score entries for sp"),
    }
}

#[get("points/sp")]
async fn get_points_sp() -> impl Responder {
    let res = read_from_file("sp").await;
    match res {
        Ok(sp_points) => HttpResponse::Ok().json(sp_points),
        _ => HttpResponse::NotFound().body("No score entries found."),
    }
}

#[post("/points/coop")]
async fn post_points_coop(data: web::Json<PointsWrapper>) -> impl Responder {
    // Cache data in .json files
    match write_to_file("coop", data).await {
        Ok(_) => HttpResponse::Ok().body("Success"), // TODO: Fix error handling (return values?)
        _ => HttpResponse::NotFound().body("Error updaing score entries for coop"),
    }
}

#[get("points/coop")]
async fn get_points_coop() -> impl Responder {
    let res = read_from_file("coop").await;
    match res {
        Ok(coop_points) => HttpResponse::Ok().json(coop_points),
        _ => HttpResponse::NotFound().body("No score entries found."),
    }
}

#[post("/points/chapter")]
async fn post_points_chapter(data: web::Json<PointsWrapper>) -> impl Responder {
    match write_to_file(&data.id.expect("No chapter ID for chapter").to_string(), data).await {
        Ok(_) => HttpResponse::Ok().body("Success"), // TODO: Fix error handling (return values?)
        _ => HttpResponse::NotFound().body("Error updaing score entries for chapter"),
    }
}

#[get("points/chapter")]
async fn get_points_chapter(data: web::Json<i32>) -> impl Responder {
    let res = read_from_file(&data.to_string()).await;
    match res {
        Ok(chapter_points) => HttpResponse::Ok().json(chapter_points),
        _ => HttpResponse::NotFound().body("No score entries found."),
    }
}

#[post("/points/overall")]
async fn post_points_overall(data: web::Json<PointsWrapper>) -> impl Responder {
    match write_to_file("overall", data).await {
        Ok(_) => HttpResponse::Ok().body("Success"), // TODO: Fix error handling (return values?)
        _ => HttpResponse::NotFound().body("Error updaing score entries for overall"),
    }
}

#[get("points/overall")]
async fn get_points_overall() -> impl Responder {
    let res = read_from_file("overall").await;
    match res {
        Ok(overall_points) => HttpResponse::Ok().json(overall_points),
        _ => HttpResponse::NotFound().body("No score entries found."),
    }
}
