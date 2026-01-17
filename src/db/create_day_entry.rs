use rusqlite::Transaction;
use rusqlite::params;

use crate::error::Fallible;
use crate::types::Date;

/// Create a day entry.
pub fn create_day_entry(tx: &Transaction<'_>, date: Date, content: &str) -> Fallible<()> {
    let sql = "
        insert into day_entries
            (date, content)
        values
            (?1, ?2);
    ";
    tx.execute(sql, params![date, content])?;
    Ok(())
}
