use serde::Deserialize;
use config::ConfigError;

// Server Config Data (from .env)
#[derive(Deserialize)]
pub struct ServerConfig{
    pub host: String,
    pub port: i32,
}
// Database Config Data (from .env)
#[derive(Deserialize)]
pub struct DatabaseConfig{
    pub database_url: String
}
// Congif Wrapper Struct
#[derive(Deserialize)]
pub struct Config{
    pub server: ServerConfig,
    pub database: DatabaseConfig
}
// Extracts the environment variables from .env
impl Config {
    pub fn from_env() -> Result<Self, ConfigError>{
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }  
}