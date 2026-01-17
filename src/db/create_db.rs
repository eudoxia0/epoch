use std::path::Path;

use rusqlite::Connection;
use rusqlite::config::DbConfig;

use crate::error::Fallible;

const SCHEMA: &str = include_str!("schema.sql");

/// Connect to the database at the given path. If it does not exist, create it.
pub fn open_db(path: &Path) -> Fallible<Connection> {
    if path.exists() {
        let conn = Connection::open(path)?;
        conn.set_db_config(DbConfig::SQLITE_DBCONFIG_ENABLE_FKEY, true)?;
        Ok(conn)
    } else {
        let conn = Connection::open(path)?;
        conn.set_db_config(DbConfig::SQLITE_DBCONFIG_ENABLE_FKEY, true)?;
        conn.execute_batch(SCHEMA)?;
        Ok(conn)
    }
}
