// Database

/// Empty struct to allow for implementation blocks for admin specific db interactions
pub struct Admin {}

/// Wrapper around an optional i32, for use in [actix_web::web::Query]
#[derive(Debug, Serialize, Deserialize)]
pub struct AdminLevel {
    pub admin_level: Option<i32>,
}
