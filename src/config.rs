use std::path::PathBuf;

use crate::error::AppError;
use crate::error::Fallible;

/// Application configuration.
#[derive(Clone)]
pub struct Config {
    /// Path to the database file.
    pub db_path: PathBuf,
    /// Server port.
    pub port: u16,
}

impl Config {
    /// Load the configuration from the environment variables.
    pub fn from_env() -> Fallible<Self> {
        let db_path: PathBuf = PathBuf::from(read_env_var("EPOCH_DB_PATH")?);
        let port: u16 = read_env_var("EPOCH_PORT")?.parse()?;
        Ok(Self { db_path, port })
    }
}

/// Fallible wrapper around env var reading.
fn read_env_var(name: &str) -> Fallible<String> {
    match std::env::var(name) {
        Ok(v) => Ok(v),
        Err(_) => Err(AppError::new(format!(
            "Failed to read value of {name} environment variable."
        ))),
    }
}
