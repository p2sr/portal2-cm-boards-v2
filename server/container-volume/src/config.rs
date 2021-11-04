use serde::Deserialize;
use config::ConfigError;

/// Server hosting information for mounting the webserver.
#[derive(Deserialize, Debug)]
pub struct ServerConfig{
    pub host: String,
    pub port: i32,
}
/// Holds database connection information.
// #[derive(Deserialize)]
// pub struct DatabaseConfig{
//     pub database_url: String
// }
/// The proof standards, update based on the mod tools desired.
#[derive(Deserialize, Debug)]
pub struct ProofConfig{
    pub demo: i32,
    pub video: i32,
}
#[derive(Deserialize, Debug)]
pub struct Config{
    pub database_url: String,
    pub server: ServerConfig,
    pub proof: ProofConfig,
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