use serde::Deserialize;
use config::ConfigError;

/// Server hosting information for mounting the webserver.
#[derive(Deserialize)]
pub struct ServerConfig{
    pub host: String,
    pub port: i32,
}
/// Holds database connection information.
#[derive(Deserialize)]
pub struct DatabaseConfig{
    pub database_url: String
}

/// Wrapper for holding configuration details from environment.
///
/// The purpose of this file is to handle loading configs from the `.env` file.
/// After pulling from github, the `.env.example` should be copied, and the `.example`
/// should be removed. Fill in the appropriate fields as per the README.MD
/// ```rust
/// pub struct ServerConfig{
///    pub host: String,
///    pub port: i32,
/// }
/// ```
/// ```rust
/// pub struct DatabaseConfig{
///    pub database_url: String
/// }
/// ```
/// # Usage
/// ```rust
/// let config = crate::config::Config::from_env().unwrap();
/// 
/// HttpServer::new(|| {
///     App::new().route("/", web::get().to(|| HttpResponse::Ok()))
///     })
///     .bind(format!("{}:{}", config.server.host, config.server.port))?
///     .run()
///     .await
/// ```
#[derive(Deserialize)]
pub struct Config{
    pub server: ServerConfig,
    pub database: DatabaseConfig
}
// Extracts the environment variables from .env
impl Config {
    /// The function fall that attempts to parse the `.env`
    pub fn from_env() -> Result<Self, ConfigError>{
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }  
}