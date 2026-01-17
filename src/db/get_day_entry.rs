use rusqlite::Transaction;
use rusqlite::params;

use crate::error::Fallible;
use crate::types::Date;

pub struct DayEntry {
    pub date: Date,
    pub content: String,
}

/// Find a day entry by its date, if it exists.
pub fn get_day_entry(tx: &Transaction<'_>, date: Date) -> Fallible<Option<DayEntry>> {
    todo!()
}
