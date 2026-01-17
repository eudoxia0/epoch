use rusqlite::Connection;
use rusqlite::config::DbConfig;

use crate::error::Fallible;

const SCHEMA: &str = include_str!("schema.sql");

/// Create a new in-memory database.
pub fn create_db() -> Fallible<Connection> {
    let conn = Connection::open_in_memory()?;
    conn.set_db_config(DbConfig::SQLITE_DBCONFIG_ENABLE_FKEY, true)?;
    conn.execute_batch(SCHEMA)?;
    Ok(conn)
}
