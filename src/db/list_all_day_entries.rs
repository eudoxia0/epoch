use rusqlite::Transaction;
use rusqlite::params;

use crate::db::get_day_entry::DayEntry;
use crate::error::Fallible;

/// List all day entries in the database, sorted by ascending date.
pub fn list_all_day_entries(tx: &Transaction<'_>) -> Fallible<Vec<DayEntry>> {
    let sql = "
        select
            date,
            content
        from
            day_entries
        order by
            date asc;
    ";
    let mut stmt = tx.prepare(sql)?;
    let rows = stmt.query_map(params![], |row| {
        Ok(DayEntry {
            date: row.get(0)?,
            content: row.get(1)?,
        })
    })?;
    let mut entries = Vec::new();
    for entry in rows {
        entries.push(entry?);
    }
    Ok(entries)
}
