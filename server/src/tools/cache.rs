use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Holds a thread-sharable hashmap that we use to control cache invalidation.
#[derive(Debug, Clone)]
pub struct CacheState {
    pub current_state: Arc<Mutex<HashMap<&'static str, bool>>>,
}
impl CacheState {
    /// Constructs a new hashmap for the cache state with static str's to represent all the values we want to cache
    pub fn new() -> Self {
        let mut hm = HashMap::new();
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
            "pointssp",
            "pointscoop",
            "pointsoverall",
        ];
        for x in cached_endpoints {
            hm.insert(x, false);
        }
        //println!("{:#?}", hm);
        CacheState {
            current_state: Arc::new(Mutex::new(hm)),
        }
    }
}
