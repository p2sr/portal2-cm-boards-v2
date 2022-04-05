use crate::models::models::{CoopMap, Maps, Points, SpMap};
use crate::tools::config::Config;
use anyhow::Result;
use serde::Serialize;
use sqlx::PgPool;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ranks {
    // profile_number -> map_id -> rank mapping
    pub current_ranks: HashMap<String, HashMap<String, i32>>,
}

/// Holds a thread-sharable hashmap that we use to control cache invalidation.
#[derive(Debug, Clone)]
pub struct CacheState {
    pub current_state: Arc<Mutex<HashMap<&'static str, bool>>>,
    pub default_cat_ids: HashMap<String, i32>,
    pub points: Arc<Mutex<HashMap<&'static str, HashMap<String, Points>>>>,
    pub ranks: Arc<Mutex<Ranks>>,
}

impl CacheState {
    /// Constructs a new hashmap for the cache state with static str's to represent all the values we want to cache
    pub async fn new(
        pool: &PgPool,
        config: &Config,
        default_cat_ids: HashMap<String, i32>,
    ) -> Self {
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
        for (i, x) in cached_endpoints.into_iter().enumerate() {
            if i >= 2 {
                match Self::load(x).await {
                    Ok(hm) => points.insert(x, hm),
                    Err(e) => {
                        // TODO: Call the backend here
                        eprintln!("Could not load {} cache from file, will need to be calculated by the backend. -> {}", x, e);
                        points.insert(x, HashMap::new())
                    }
                };
            } else {
                hm.insert(x, false);
            }
        }

        let current_ranks = CacheState::load_all_ranks(&default_cat_ids, pool, config, true)
            .await
            .unwrap();

        CacheState {
            current_state: Arc::new(Mutex::new(hm)),
            default_cat_ids,
            points: Arc::new(Mutex::new(points)),
            ranks: Arc::new(Mutex::new(current_ranks)),
        }
    }

    async fn load(x: &'static str) -> Result<HashMap<String, Points>> {
        Ok(read_from_file::<HashMap<String, Points>>(x).await?)
    }

    pub async fn load_all_ranks(
        default_cat_ids: &HashMap<String, i32>,
        pool: &PgPool,
        config: &Config,
        try_from_file: bool,
    ) -> Result<Ranks> {
        // use std::time::Instant;
        // let now = Instant::now();
        let id = "ranks";
        // Try to load using a file on startup.
        if try_from_file {
            match read_from_file::<Ranks>(id).await {
                Ok(r) => {
                    // let elapsed = now.elapsed();
                    // println!("Elapsed: {:.2?}", elapsed);
                    return Ok(r);
                }
                Err(e) => {
                    eprintln!("Error grabbing rank cache from file -> {}", e);
                    ()
                }
            }
        }

        let coop = Maps::get_steam_ids(pool, true).await?;
        let sp = Maps::get_steam_ids(pool, false).await?;
        let mut current_ranks = HashMap::with_capacity(1000);
        for map in sp {
            let res =
                SpMap::get_sp_map_page(pool, &map, config.proof.results, default_cat_ids[&map])
                    .await?;
            for (i, entry) in res.into_iter().enumerate() {
                let user = current_ranks
                    .entry(entry.profile_number)
                    .or_insert(HashMap::new());
                user.insert(map.clone(), (i + 1) as i32);
            }
        }
        for map in coop {
            let res =
                CoopMap::get_coop_map_page(pool, &map, config.proof.results, default_cat_ids[&map])
                    .await?;
            for (i, entry) in res.into_iter().enumerate() {
                let user = current_ranks
                    .entry(entry.profile_number1)
                    .or_insert(HashMap::new());
                if user.get(&map).is_none() {
                    user.insert(map.clone(), (i + 1) as i32);
                }
                let user = current_ranks
                    .entry(entry.profile_number2)
                    .or_insert(HashMap::new());
                if user.get(&map).is_none() {
                    user.insert(map.clone(), (i + 1) as i32);
                }
            }
        }
        let fin = Ranks { current_ranks };
        write_to_file(id, &fin).await.unwrap();
        // let elapsed = now.elapsed();
        // println!("Elapsed: {:.2?}", elapsed);
        Ok(fin)
    }
    // TODO: Testing
    pub async fn reload_rank(
        &self,
        pool: &PgPool,
        map_id: &String,
        config: &Config,
        is_coop: bool,
    ) -> () {
        if is_coop {
            let res = CoopMap::get_coop_map_page(
                pool,
                map_id,
                config.proof.results,
                self.default_cat_ids[map_id],
            )
            .await
            .unwrap();
            let r = &mut self.ranks.lock().await;
            // TOOD:
            // This logic is super unfortunate, we cannot reuse the logic for the inital setup because
            // the user will most likely already exist in our hashmap. This means we have to allocate
            // **another** hashmap, this is really unfortunate, and I want to fix it somehow in the future.
            let mut checker = HashSet::with_capacity(500);

            for (i, entry) in res.into_iter().enumerate() {
                if !checker.insert(entry.profile_number1.clone()) {
                    let user = r
                        .current_ranks
                        .entry(entry.profile_number1)
                        .or_insert(HashMap::new());
                    if user.get(map_id).is_none() {
                        user.insert(map_id.clone(), (i + 1) as i32);
                    }
                }
                if !checker.insert(entry.profile_number2.clone()) {
                    let user = r
                        .current_ranks
                        .entry(entry.profile_number2)
                        .or_insert(HashMap::new());
                    if user.get(map_id).is_none() {
                        user.insert(map_id.clone(), (i + 1) as i32);
                    }
                }
            }
        } else {
            let res = SpMap::get_sp_map_page(
                pool,
                map_id,
                config.proof.results,
                self.default_cat_ids[map_id],
            )
            .await
            .unwrap();
            let r = &mut self.ranks.lock().await;
            for (i, entry) in res.into_iter().enumerate() {
                let user = r
                    .current_ranks
                    .entry(entry.profile_number)
                    .or_insert(HashMap::new());
                user.insert(map_id.clone(), (i + 1) as i32);
            }
        }
    }
}

/// Writes data to a file if the type implements Serialize
pub async fn write_to_file<T: Serialize>(id: &str, data: &T) -> Result<()> {
    use std::fs;
    fs::create_dir_all("./cache")?;
    let path_str = format!("./cache/{}.json", id);
    let path = Path::new(&path_str);
    serde_json::to_writer(&File::create(path)?, data)
        .map(|_| ())
        .map_err(|err| err.into())
}

// Reads data from a file for any type that implements Deserialize
pub async fn read_from_file<T: for<'de> serde::Deserialize<'de>>(id: &str) -> Result<T> {
    let path_str = format!("./cache/{}.json", id);
    let path = Path::new(&path_str);
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let res: T = serde_json::from_reader(reader)?;
    Ok(res)
}
