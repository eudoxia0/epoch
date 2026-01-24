use rusqlite::Transaction;
use rusqlite::params;

use crate::error::Fallible;
use crate::types::Date;
use crate::types::EntryText;

pub struct DayEntry {
    pub date: Date,
    pub content: EntryText,
}

impl DayEntry {
    /// Create a day entry.
    pub fn create(tx: &Transaction<'_>, date: Date, content: &EntryText) -> Fallible<()> {
        let word_count: i32 = content.word_count();
        let sql = "
            insert into day_entries
                (date, content, word_count)
            values
                (?1, ?2, ?3);
        ";
        tx.execute(sql, params![date, content, word_count])?;
        Ok(())
    }

    /// Find a day entry by date, if it exists.
    pub fn get_opt(tx: &Transaction<'_>, date: Date) -> Fallible<Option<DayEntry>> {
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

    /// List all day entries.
    pub fn list(tx: &Transaction<'_>) -> Fallible<Vec<DayEntry>> {
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

    /// Search day entries.
    pub fn search(tx: &Transaction<'_>, query: &str) -> Fallible<Vec<Date>> {
        let sql = "
            select
                date
            from
                day_entries
            where
                lower(content) like lower(?1)
            order by
                date asc;
        ";
        let mut stmt = tx.prepare(sql)?;
        let rows = stmt.query_map(params![format!("%{}%", query)], |row| row.get(0))?;
        let mut entries = Vec::new();
        for entry in rows {
            entries.push(entry?);
        }
        Ok(entries)
    }

    /// Return the total number of day entries.
    pub fn count(tx: &Transaction<'_>) -> Fallible<i32> {
        let sql = "
            select
                count(*)
            from
                day_entries;
        ";
        let result: i32 = tx.query_one(sql, [], |row| row.get(0))?;
        Ok(result)
    }

    /// Return the total number of words across all entries.
    pub fn count_words(tx: &Transaction<'_>) -> Fallible<i32> {
        let sql = "
            select
                coalesce(sum(word_count), 0)
            from
                day_entries;
        ";
        let result: i32 = tx.query_one(sql, [], |row| row.get(0))?;
        Ok(result)
    }

    /// Update a day entry.
    pub fn update(tx: &Transaction<'_>, date: Date, content: &EntryText) -> Fallible<()> {
        let word_count: i32 = content.word_count();
        let sql = "
            update day_entries set
                content    = ?2,
                word_count = ?3
            where
                date = ?1;
        ";
        tx.execute(sql, params![date, content, word_count])?;
        Ok(())
    }
}
