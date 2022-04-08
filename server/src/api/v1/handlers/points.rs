use crate::models::models::{Points, PointsReadWrapper, PointsReceiveWrapper, PointsWriteWrapper};
use crate::tools::cache::{write_to_file, CacheState};
use actix_web::{get, post, web, HttpResponse, Responder};
use anyhow::{Error, Result};
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Writes out json data to cache points for the boards.
pub async fn write_points_to_file(
    id: &str,
    data: &web::Json<PointsReceiveWrapper>,
) -> Result<(), Error> {
    use std::fs;
    fs::create_dir_all("./points")?;
    let path_str = format!("./points/{}.json", id);
    let path = Path::new(&path_str);
    let write = PointsWriteWrapper {
        id: data.id,
        points: &data.ordered_points,
    };
    serde_json::to_writer(&File::create(path)?, &write)
        .map(|_| ())
        .map_err(|err| err.into())
}

/// Reads in json from the cache for the passed in ID.
pub async fn read_points_from_file(id: &str) -> Result<PointsReadWrapper, Error> {
    let path_str = format!("./points/{}.json", id);
    let path = Path::new(&path_str);
    let mut file = File::open(path)?;
    let mut buff = String::new();
    file.read_to_string(&mut buff)?;
    let res: PointsReadWrapper = serde_json::from_str(&buff)?;
    Ok(res)
}

/// Update single player points data.
#[post("/points/sp")]
async fn points_sp_add(
    data: web::Json<PointsReceiveWrapper>,
    cache: web::Data<CacheState>,
) -> impl Responder {
    // Cache data in .json files
    match write_points_to_file("sp", &data).await {
        Ok(_) => {
            let id = "points_sp";
            let points_hm = &mut cache.points.lock().await;
            let points_cache = points_hm.get_mut(&id).unwrap();
            for (k, v) in data.into_inner().hm_points.into_iter() {
                points_cache.insert(k, v);
            }
            write_to_file(&id, &points_cache).await.unwrap();
            // println!("Updated cache.");
            HttpResponse::Ok().body("Success")
        }
        _ => HttpResponse::NotFound().body("Error updaing score entries for sp"),
    }
}

/// Gget single player points data.
#[get("points/sp")]
async fn points_sp() -> impl Responder {
    let res = read_points_from_file("sp").await;
    match res {
        Ok(sp_points) => HttpResponse::Ok().json(sp_points),
        _ => HttpResponse::NotFound().body("No score entries for SP found."),
    }
}

/// Update coop points data.
#[post("/points/coop")]
async fn points_coop_add(
    data: web::Json<PointsReceiveWrapper>,
    cache: web::Data<CacheState>,
) -> impl Responder {
    // Cache data in .json files
    match write_points_to_file("coop", &data).await {
        Ok(_) => {
            let id = "points_coop";
            let points_hm = &mut cache.points.lock().await;
            let points_cache = points_hm.get_mut(&id).unwrap();
            for (k, v) in data.into_inner().hm_points.into_iter() {
                points_cache.insert(k, v);
            }
            write_to_file(&id, &points_cache).await.unwrap();
            // println!("Updated cache.");
            HttpResponse::Ok().body("Success")
        }
        _ => HttpResponse::NotFound().body("Error updaing score entries for coop"),
    }
}

/// Get coop points data.
#[get("points/coop")]
async fn points_coop() -> impl Responder {
    let res = read_points_from_file("coop").await;
    match res {
        Ok(coop_points) => HttpResponse::Ok().json(coop_points),
        _ => HttpResponse::NotFound().body("No score entries found."),
    }
}

/// Update chapter data, uses JSON ID (see [PointsReceiveWrapper]).
#[post("/points/chapter")]
async fn points_chapter_add(
    data: web::Json<PointsReceiveWrapper>,
    cache: web::Data<CacheState>,
) -> impl Responder {
    let id = data.id.expect("No chapter ID for chapter").to_string();
    match write_points_to_file(&id, &data).await {
        Ok(_) => {
            let id_ = format!("points{}", id);
            let points_hm = &mut cache.points.lock().await;
            let points_cache = points_hm.get_mut(&*id_).unwrap();
            for (k, v) in data.into_inner().hm_points.into_iter() {
                points_cache.insert(k, v);
            }
            write_to_file(&id_, &points_cache).await.unwrap();
            // println!("Updated cache.");
            HttpResponse::Ok().body("Success")
        }
        _ => HttpResponse::NotFound().body("Error updaing score entries for chapter"),
    }
}

/// Get points data for a specific chapter.
#[get("points/chapter/{id}")]
async fn points_chapter(id: web::Path<u64>) -> impl Responder {
    let res = read_points_from_file(&id.to_string()).await;
    match res {
        Ok(chapter_points) => HttpResponse::Ok().json(chapter_points),
        _ => HttpResponse::NotFound().body("No coop score entries found."),
    }
}

/// Update overall points data.
#[post("/points/overall")]
async fn points_overall_add(
    data: web::Json<PointsReceiveWrapper>,
    cache: web::Data<CacheState>,
) -> impl Responder {
    match write_points_to_file("overall", &data).await {
        Ok(_) => {
            let id = "points_overall";
            let points_hm = &mut cache.points.lock().await;
            let points_cache = points_hm.get_mut(&id).unwrap();
            for (k, v) in data.into_inner().hm_points.into_iter() {
                points_cache.insert(k, v);
            }
            write_to_file(&id, &points_cache).await.unwrap();
            println!("{:#?}", points_cache);
            HttpResponse::Ok().body("Success")
        }
        _ => HttpResponse::NotFound().body("Error updaing score entries for overall"),
    }
}

/// Get overall points data.
#[get("points/overall")]
async fn points_overall() -> impl Responder {
    let res = read_points_from_file("overall").await;
    match res {
        Ok(overall_points) => HttpResponse::Ok().json(overall_points),
        _ => HttpResponse::NotFound().body("No score entries found."),
    }
}
