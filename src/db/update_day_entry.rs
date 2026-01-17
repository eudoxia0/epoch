use rusqlite::Transaction;
use rusqlite::params;

use crate::error::Fallible;
use crate::types::Date;
use crate::types::EntryText;

/// Update a day entry.
pub fn update_day_entry(tx: &Transaction<'_>, date: Date, content: &EntryText) -> Fallible<()> {
    let sql = "
        update day_entries set
            content = ?2
        where
            date = ?1;
    ";
    tx.execute(sql, params![date, content])?;
    Ok(())
}
