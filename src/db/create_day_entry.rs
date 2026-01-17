use rusqlite::Transaction;

use crate::error::Fallible;

/// Create a day entry.
pub fn create_day_entry(tx: &Transaction<'_>) -> Fallible<()> {
    todo!()
}
