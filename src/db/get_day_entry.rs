use rusqlite::Transaction;
use rusqlite::params;

use crate::error::Fallible;
use crate::types::Date;
use crate::types::EntryText;

/// The contents of a day entry.
pub struct DayEntry {
    pub date: Date,
    pub content: EntryText,
}

/// Find a day entry by its date, if it exists.
pub fn get_day_entry(tx: &Transaction<'_>, date: Date) -> Fallible<Option<DayEntry>> {
    let sql = "
        select
            content
        from
            day_entries
        where
            date = ?1;
    ";
    let mut stmt = tx.prepare(sql)?;
    let mut rows = stmt.query_map(params![date], |row| row.get(0))?;
    if let Some(row) = rows.next() {
        let content: EntryText = row?;
        Ok(Some(DayEntry { date, content }))
    } else {
        Ok(None)
    }
}
