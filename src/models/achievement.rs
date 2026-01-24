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
        todo!()
    }

    /// List all achievements in most-recent-first order.
    pub fn list(tx: &Transaction<'_>) -> Fallible<Vec<Achievement>> {
        todo!()
    }

    /// Delete an achievement.
    pub fn delete(tx: &Transaction<'_>, achievement_id: AchievementId) -> Fallible<()> {
        todo!()
    }
}
