//! Caching implementations specific to the board.
//!
//! Currently built off [std::collections::HashMap].
//!
//! ## Accessing in endpoints.
//! ```rust
//! use crate::tools::cache::CacheState;
//! use actix_web::{get, web, HttpResponse, Responder};
//!
//! #[get("/test")]
//! async fn test(cache: web::Data<CacheState>) -> impl Responder {
//!     let cache = cache.into_inner(); // Extracts the CacheState from the [actix_web::web::Data] wrapper
//!     // Access the default category ids.
//!     let map_id = "47458";
//!     let def_cat_id = cache.default_cat_ids[map_id];
//!
//!     // Check the current cache state for points.
//!     let state_data = &mut cache.current_state.lock().await; // We use &mut here so that we can change the value accordingly.
//!     // Check the cache state for "coop_previews"
//!     // Check `cached_endpoints` in CacheState::new() for which endpoints are current cached.
//!     let is_cached = state_data.get_mut("coop_previews").unwrap();
//!     // Derefernce the bool and match accordingly.
//!     if !*is_cached {
//!         // Assume we cache here, to see full example check [crate::server::api::v1::handlers::coop::get_cooperative_preview]
//!         *is_cached = true;
//!     }
//!
//!     // Check points which maps as follows:
//!     // Points descriptor str -> profile_number -> [crate::models::points::Points]
//!     let points_id = "points_sp";
//!     let points_hm = &mut cache.points.lock().await;
//!     // This gives us a &mut HashMap that maps profile_number to Points struct
//!     // For full example see [server::api::v1::handlers::points::post_points_sp]
//!     let points_cache = points_hm.get_mut(points_id).unwrap();
//!
//!     // Use Ranks
//!     let profile_number = "76561198135023038".to_string();
//!     let ranks = &mut cache.ranks.lock().await;
//!     // Accessing current_ranks field, then using as a hashmap with the mapping from map_id -> rank
//!     let user = ranks
//!         .current_ranks
//!         .entry(profile_number)
//!         .or_insert_with(HashMap::new);
//!
//!     HttpResponse::Ok().body("test")
//! }
//! ```
//!
use crate::{
    models::{coop::CoopMap, maps::Maps, points::Points, sp::SpMap},
    tools::config::Config,
};
use anyhow::Result;
use serde::Serialize;
use sqlx::PgPool;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::BufReader,
    path::Path,
    sync::Arc,
};
use tokio::sync::Mutex;

pub const SP_PREVIEWS: &'static str = "sp_previews";
pub const COOP_PREVIEWS: &'static str = "coop_previews";
pub const POINTS_1: &'static str = "points1";
pub const POINTS_2: &'static str = "points2";
pub const POINTS_3: &'static str = "points3";
pub const POINTS_4: &'static str = "points4";
pub const POINTS_5: &'static str = "points5";
pub const POINTS_6: &'static str = "points6";
pub const POINTS_7: &'static str = "points7";
pub const POINTS_8: &'static str = "points8";
pub const POINTS_9: &'static str = "points9";
pub const POINTS_10: &'static str = "points10";
pub const POINTS_11: &'static str = "points11";
pub const POINTS_12: &'static str = "points12";
pub const POINTS_13: &'static str = "points13";
pub const POINTS_14: &'static str = "points14";
pub const POINTS_15: &'static str = "points15";
pub const POINTS_SP: &'static str = "points_sp";
pub const POINTS_COOP: &'static str = "points_coop";
pub const POINTS_OVERALL: &'static str = "points_overall";

/// Cache for the current ranks all players have within the top X scores (defined by [crate::tools::config::ProofConfig])
///
/// The mapping is as follows:
/// - profile_number -> map_id -> rank
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ranks {
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
    ///
    /// ## Current list of cached endpoints.
    ///
    ///  ```
    ///  "sp_previews", "coop_previews", "points1", "points2", "points3",
    ///  "points4", "points5", "points6", "points7", "points8", "points9",
    ///  "points10", "points11", "points12", "points13", "points14", "points15",
    ///  "points_sp", "points_coop", "points_overall"
    /// ```
    ///
    /// **NOTE**: Portal 2 references coop chapters 1-6 as chapter ID's 1-6, meaning 1-6 are coop, and 7-15 are SP.
    pub async fn new(
        pool: &PgPool,
        config: &Config,
        default_cat_ids: HashMap<String, i32>,
    ) -> Self {
        let mut hm = HashMap::new();
        let mut points = HashMap::new();
        let cached_endpoints: Vec<&'static str> = vec![
            SP_PREVIEWS,
            COOP_PREVIEWS,
            POINTS_1,
            POINTS_2,
            POINTS_3,
            POINTS_4,
            POINTS_5,
            POINTS_6,
            POINTS_7,
            POINTS_8,
            POINTS_9,
            POINTS_10,
            POINTS_11,
            POINTS_12,
            POINTS_13,
            POINTS_14,
            POINTS_15,
            POINTS_SP,
            POINTS_COOP,
            POINTS_OVERALL,
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
    /// Try to load points data from files rather than expecting that the backend must send over the data fresh every time the web server is run.
    async fn load(x: &'static str) -> Result<HashMap<String, Points>> {
        read_from_file::<HashMap<String, Points>>(x).await
    }
    /// Create a fresh set of ranks to cache. Takes a good amount of time to go through all 108 maps and populate ranks for all.
    ///
    /// We prefer to try and use `reload_rank` where possible to reload ranks on individual maps.
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
                }
            }
        }

        let coop = Maps::get_steam_ids(pool, true).await?;
        let sp = Maps::get_steam_ids(pool, false).await?;
        let mut current_ranks = HashMap::with_capacity(1000);
        for map in sp {
            let res =
                SpMap::get_sp_map_page(pool, &map, config.proof.results, default_cat_ids[&map], 1)
                    .await?;
            for (i, entry) in res.into_iter().enumerate() {
                let user = current_ranks
                    .entry(entry.profile_number)
                    .or_insert_with(HashMap::new);
                user.insert(map.clone(), (i + 1) as i32);
            }
        }
        for map in coop {
            let res = CoopMap::get_coop_map_page(
                pool,
                &map,
                config.proof.results,
                default_cat_ids[&map],
                1,
            )
            .await?;
            for (i, entry) in res.into_iter().enumerate() {
                let user = current_ranks
                    .entry(entry.profile_number1)
                    .or_insert_with(HashMap::new);
                if user.get(&map).is_none() {
                    user.insert(map.clone(), (i + 1) as i32);
                }
                let user = current_ranks
                    .entry(entry.profile_number2)
                    .or_insert_with(HashMap::new);
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
    /// Refreshes map rank cache on a specific map. Especially slow for coop, but faster than refreshing all maps.
    #[allow(dead_code)]
    pub async fn reload_rank(
        &self,
        pool: &PgPool,
        map_id: &String,
        config: &Config,
        is_coop: bool,
    ) {
        if is_coop {
            let res = CoopMap::get_coop_map_page(
                pool,
                map_id,
                config.proof.results,
                self.default_cat_ids[map_id],
                1,
            )
            .await
            .unwrap();
            let r = &mut self.ranks.lock().await;
            // TODO:
            // This logic is super unfortunate, we cannot reuse the logic for the inital setup because
            // the user will most likely already exist in our hashmap. This means we have to allocate
            // **another** hashmap, this is really unfortunate, and I want to fix it somehow in the future.
            let mut checker = HashSet::with_capacity(500);

            for (i, entry) in res.into_iter().enumerate() {
                if !checker.insert(entry.profile_number1.clone()) {
                    let user = r
                        .current_ranks
                        .entry(entry.profile_number1)
                        .or_insert_with(HashMap::new);
                    if user.get(map_id).is_none() {
                        user.insert(map_id.clone(), (i + 1) as i32);
                    }
                }
                if !checker.insert(entry.profile_number2.clone()) {
                    let user = r
                        .current_ranks
                        .entry(entry.profile_number2)
                        .or_insert_with(HashMap::new);
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
                1,
            )
            .await
            .unwrap();
            let r = &mut self.ranks.lock().await;
            for (i, entry) in res.into_iter().enumerate() {
                let user = r
                    .current_ranks
                    .entry(entry.profile_number)
                    .or_insert_with(HashMap::new);
                user.insert(map_id.clone(), (i + 1) as i32);
            }
        }
    }
    #[allow(dead_code)]
    pub async fn update_current_state(&self, update: &'static str, set_cache: bool) -> () {
        let state_data = &mut self.current_state.lock().await;
        let is_cached = state_data.get_mut(update).unwrap();
        *is_cached = set_cache;
    }
    pub async fn update_current_states(&self, update: &[&'static str], set_cache: &[bool]) -> () {
        assert_eq!(update.len(), set_cache.len());
        let state_data = &mut self.current_state.lock().await;
        for (i, x) in update.into_iter().enumerate() {
            let is_cached = state_data.get_mut(x).unwrap();
            *is_cached = set_cache[i];
        }
    }
    pub async fn get_current_state(&self, value: &'static str) -> bool {
        let state_data = &mut self.current_state.lock().await;
        *state_data.get_mut(value).unwrap()
    }
}

/// Writes data to a file if the type implements [serde::Serialize]
///
/// The function takes an `id` that will be used to find a file in the following path:
/// - `./cachce/{id}.json`
///
/// The ./cache/ is relative to the `server` directory, above src level.
/// ## Example
/// ```rust
/// #[derive(Serialize, Debug)]
/// pub struct A {
///     a: &str,
/// }
///
/// async fn test() {
///     let id = "test"; // Will try to write to `./cache/test.json`
///     let test_val = A { a: "hello world" };
///     match write_to_file::<A
/// >(id, test_val).await {
///         Ok(()) => println!("Success!"),
///         Err(e) => eprintln!("Error -> {}", e),
///     }
/// }
/// ```
pub async fn write_to_file<T: Serialize>(id: &str, data: &T) -> Result<()> {
    use std::fs;
    fs::create_dir_all("./cache")?;
    let path_str = format!("./cache/{}.json", id);
    let path = Path::new(&path_str);
    serde_json::to_writer(&File::create(path)?, data)
        .map(|_| ())
        .map_err(|err| err.into())
}

// TODO: Have the id be generic over any type that implements [std::fmt]
/// Reads data from a file for any type that implements [serde::Deserialize]
///
/// The function takes an `id` that will be used to find a file in the following path:
/// - `./cachce/{id}.json`
///
/// The ./cache/ is relative to the `server` directory, above src level.
/// ## Example
/// ```rust
/// #[derive(Deserialize, Debug)]
/// pub struct A {
///     a: i32,
/// }
///
/// async fn test() {
///     let id = "test"; // Will try to open `./cache/test.json`
///     match read_from_file::<A>(id).await {
///         Ok(a) => println!("{:#?}", a),
///         Err(e) => eprintln!("Error -> {}", e),
///     }
/// }
/// ```
pub async fn read_from_file<T: for<'de> serde::Deserialize<'de>>(id: &str) -> Result<T> {
    let path_str = format!("./cache/{}.json", id);
    let path = Path::new(&path_str);
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let res: T = serde_json::from_reader(reader)?;
    Ok(res)
}
