use crate::models::models::Points;
use anyhow::Error;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Holds a thread-sharable hashmap that we use to control cache invalidation.
#[derive(Debug, Clone)]
pub struct CacheState {
    pub current_state: Arc<Mutex<HashMap<&'static str, bool>>>,
    pub default_cat_ids: HashMap<String, i32>,
    pub points: Arc<Mutex<HashMap<&'static str, HashMap<String, Points>>>>,
}
impl CacheState {
    /// Constructs a new hashmap for the cache state with static str's to represent all the values we want to cache
    pub fn new(default_cat_ids: HashMap<String, i32>) -> Self {
        let mut hm = HashMap::new();
        let mut points = HashMap::new();
        let cached_endpoints: Vec<&'static str> = vec![
            "sp_previews",
            "coop_previews",
            "points1",
            "points2",
            "points3",
            "points4",
            "points5",
            "points6",
            "points7",
            "points8",
            "points9",
            "points10",
            "points11",
            "points12",
            "points13",
            "points14",
            "points15",
            "points_sp",
            "points_coop",
            "points_overall",
        ];
        for x in cached_endpoints {
            if x != "sp_previews" || x != "coop_previews" {
                points.insert(x, HashMap::new());
            } else {
                hm.insert(x, false);
            }
        }
        // TODO: Dump the cache for in-memory, then load the most recent cache on startup
        CacheState {
            current_state: Arc::new(Mutex::new(hm)),
            default_cat_ids,
            points: Arc::new(Mutex::new(points)),
        }
    }
}

/// Writes data to a file if the type implements Serialize
pub async fn write_to_file<T: Serialize>(id: &str, data: &T) -> Result<(), Error> {
    use std::fs;
    fs::create_dir_all("./cache")?;
    let path_str = format!("./cache/{}.json", id);
    let path = Path::new(&path_str);
    serde_json::to_writer(&File::create(path)?, data)
        .map(|_| ())
        .map_err(|err| err.into())
}

// Reads data from a file for any type that implements Deserialize
pub async fn read_from_file<T: for<'de> serde::Deserialize<'de>>(id: &str) -> Result<T, Error> {
    let path_str = format!("./cache/{}.json", id);
    let path = Path::new(&path_str);
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let res: T = serde_json::from_reader(reader)?;
    Ok(res)
}
