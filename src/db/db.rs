use std::path::PathBuf;
use std::sync::Arc;

use rusqlite::Connection;
use rusqlite::Transaction;
use rusqlite::config::DbConfig;
use tokio::sync::Mutex;
use tokio::sync::MutexGuard;

use crate::error::Fallible;
use crate::models::Achievement;
use crate::models::DayEntry;
use crate::types::Date;
use crate::types::RichText;

const SCHEMA: &str = include_str!("schema.sql");

/// The database object serializes database access. Each method creates a
/// database connection anew, allowing the database to be [swapped out][a]
/// between requests.
///
/// [a]: https://borretti.me/article/a-syncthing-and-sqlite-gotcha
#[derive(Clone)]
pub struct Database {
    /// Path to the database file.
    path: PathBuf,
    /// Mutex lock.
    mutex: Arc<Mutex<()>>,
}

/// Data for a day entry.
pub struct DayEntryData {
    pub content: RichText,
    pub achievements: Vec<Achievement>,
}

impl Database {
    /// Construct a [`Database`] object. Creates the database at the given path
    /// if it does not exist.
    pub fn new(path: PathBuf) -> Fallible<Self> {
        if !path.exists() {
            let conn = Connection::open(&path)?;
            conn.set_db_config(DbConfig::SQLITE_DBCONFIG_ENABLE_FKEY, true)?;
            conn.execute_batch(SCHEMA)?;
        }
        let mutex: Arc<Mutex<()>> = Arc::new(Mutex::new(()));
        Ok(Self { path, mutex })
    }

    /// Find a day entry by date, if it exists.
    pub async fn get_day_entry(&self, date: Date) -> Fallible<Option<DayEntryData>> {
        let _permit: MutexGuard<'_, ()> = self.mutex.lock().await;
        let mut conn: Connection = self.open_db()?;
        let tx: Transaction<'_> = conn.transaction()?;
        let maybe_day_entry: Option<DayEntry> = DayEntry::get_opt(&tx, date)?;
        match maybe_day_entry {
            Some(de) => {
                let achievements: Vec<Achievement> = Achievement::list_by_date(&tx, date)?;
                let output: DayEntryData = DayEntryData {
                    content: de.content,
                    achievements,
                };
                Ok(Some(output))
            }
            None => Ok(None),
        }
    }

    fn open_db(&self) -> Fallible<Connection> {
        let conn: Connection = Connection::open(&self.path)?;
        conn.set_db_config(DbConfig::SQLITE_DBCONFIG_ENABLE_FKEY, true)?;
        Ok(conn)
    }
}
