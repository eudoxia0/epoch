use rusqlite::Transaction;

use crate::error::Fallible;
use crate::types::AchievementId;
use crate::types::AchievementType;
use crate::types::Date;
use crate::types::RichText;

/// Represents an achievement: something the user did that's worth recording as
/// a victory.
pub struct Achievement {
    /// The achievement ID.
    pub achievement_id: AchievementId,
    /// The date when the achievement was logged.
    pub date: Date,
    /// The achievement type.
    pub achievement_type: AchievementType,
    /// The text explaining what was achieved.
    pub content: RichText,
}

impl Achievement {
    /// Create an achievement.
    pub fn create(
        tx: &Transaction<'_>,
        date: Date,
        achievement_type: AchievementType,
        content: RichText,
    ) -> Fallible<AchievementId> {
        let sql = "
            insert into achievements
                (date, achievement_type, content)
            values
                (?1, ?2, ?3);
        ";
        tx.execute(sql, rusqlite::params![date, achievement_type, content])?;
        let achievement_id = AchievementId::new(tx.last_insert_rowid() as i32);
        Ok(achievement_id)
    }

    /// List all achievements in most-recent-first order.
    pub fn list(tx: &Transaction<'_>) -> Fallible<Vec<Achievement>> {
        let sql = "
            select
                achievement_id,
                date,
                achievement_type,
                content
            from
                achievements
            order by
                date desc,
                achievement_id desc;
        ";
        let mut stmt = tx.prepare(sql)?;
        let rows = stmt.query_map(rusqlite::params![], |row| {
            Ok(Achievement {
                achievement_id: row.get(0)?,
                date: row.get(1)?,
                achievement_type: row.get(2)?,
                content: row.get(3)?,
            })
        })?;
        let mut achievements = Vec::new();
        for achievement in rows {
            achievements.push(achievement?);
        }
        Ok(achievements)
    }

    /// List all achievements for a specific date.
    pub fn list_by_date(tx: &Transaction<'_>, date: Date) -> Fallible<Vec<Achievement>> {
        let sql = "
            select
                achievement_id,
                date,
                achievement_type,
                content
            from
                achievements
            where
                date = ?1
            order by
                achievement_id asc;
        ";
        let mut stmt = tx.prepare(sql)?;
        let rows = stmt.query_map(rusqlite::params![date], |row| {
            Ok(Achievement {
                achievement_id: row.get(0)?,
                date: row.get(1)?,
                achievement_type: row.get(2)?,
                content: row.get(3)?,
            })
        })?;
        let mut achievements = Vec::new();
        for achievement in rows {
            achievements.push(achievement?);
        }
        Ok(achievements)
    }

    /// Get a single achievement by ID.
    pub fn get(tx: &Transaction<'_>, achievement_id: AchievementId) -> Fallible<Achievement> {
        let sql = "
            select
                achievement_id,
                date,
                achievement_type,
                content
            from
                achievements
            where
                achievement_id = ?1;
        ";
        let achievement = tx.query_row(sql, rusqlite::params![achievement_id], |row| {
            Ok(Achievement {
                achievement_id: row.get(0)?,
                date: row.get(1)?,
                achievement_type: row.get(2)?,
                content: row.get(3)?,
            })
        })?;
        Ok(achievement)
    }

    /// Update an existing achievement.
    pub fn update(
        tx: &Transaction<'_>,
        achievement_id: AchievementId,
        date: Date,
        achievement_type: AchievementType,
        content: RichText,
    ) -> Fallible<()> {
        let sql = "
            update achievements
            set
                date = ?2,
                achievement_type = ?3,
                content = ?4
            where
                achievement_id = ?1;
        ";
        tx.execute(
            sql,
            rusqlite::params![achievement_id, date, achievement_type, content],
        )?;
        Ok(())
    }

    /// Delete an achievement.
    pub fn delete(tx: &Transaction<'_>, achievement_id: AchievementId) -> Fallible<()> {
        let sql = "
            delete from achievements
            where achievement_id = ?1;
        ";
        tx.execute(sql, rusqlite::params![achievement_id])?;
        Ok(())
    }
}
