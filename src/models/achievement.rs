use crate::types::AchievementId;
use crate::types::AchievementType;
use crate::types::Date;
use crate::types::RichText;

pub struct Achievement {
    pub achievement_id: AchievementId,
    pub date: Date,
    pub achievement_type: AchievementType,
    pub content: RichText,
}
