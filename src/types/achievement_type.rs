use rusqlite::ToSql;
use rusqlite::types::FromSql;
use rusqlite::types::FromSqlError;
use rusqlite::types::FromSqlResult;
use rusqlite::types::ToSqlOutput;
use rusqlite::types::ValueRef;

use crate::error::AppError;
use crate::error::Fallible;

#[derive(Clone, Copy)]
pub enum AchievementType {
    /// Finished a book.
    FinishedBook,
    /// Completed a project.
    CompletedProject,
    /// Went to the gym.
    Gym,
    /// Did something unpleasant but necessary.
    AteTheFrog,
}

impl AchievementType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FinishedBook => "finished_book",
            Self::CompletedProject => "completed_project",
            Self::Gym => "gym",
            Self::AteTheFrog => "frog",
        }
    }

    pub fn icon(self) -> &'static str {
        match self {
            Self::FinishedBook => "📚️",
            Self::CompletedProject => "🎨",
            Self::Gym => "💪",
            Self::AteTheFrog => "🐸",
        }
    }
}

impl TryFrom<&str> for AchievementType {
    type Error = AppError;

    fn try_from(value: &str) -> Fallible<Self> {
        match value {
            "finished_book" => Ok(Self::FinishedBook),
            "completed_project" => Ok(Self::CompletedProject),
            "gym" => Ok(Self::Gym),
            "frog" => Ok(Self::AteTheFrog),
            _ => Err(AppError::new(format!(
                "Invalid value for AchievementType: '{value}'."
            ))),
        }
    }
}

impl ToSql for AchievementType {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let str = self.as_str();
        Ok(ToSqlOutput::from(str))
    }
}

impl FromSql for AchievementType {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let string: String = FromSql::column_result(value)?;
        AchievementType::try_from(string.as_str()).map_err(|e| FromSqlError::Other(Box::new(e)))
    }
}
