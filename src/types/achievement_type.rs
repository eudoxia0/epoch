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
    /// Did something unpleasant but necessary.
    AteTheFrog,
    /// Reached inbox zero.
    InboxZero,
}

impl AchievementType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FinishedBook => "finished_book",
            Self::CompletedProject => "completed_project",
            Self::AteTheFrog => "frog",
            Self::InboxZero => "inbox_zero",
        }
    }

    pub fn icon(self) -> &'static str {
        match self {
            Self::FinishedBook => "📚️",
            Self::CompletedProject => "🎨",
            Self::AteTheFrog => "🐸",
            Self::InboxZero => "📧",
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Self::FinishedBook => "Finished a book",
            Self::CompletedProject => "Completed a project",
            Self::AteTheFrog => "Ate the frog",
            Self::InboxZero => "Reached inbox zero",
        }
    }
}

impl TryFrom<&str> for AchievementType {
    type Error = AppError;

    fn try_from(value: &str) -> Fallible<Self> {
        match value {
            "finished_book" => Ok(Self::FinishedBook),
            "completed_project" => Ok(Self::CompletedProject),
            "frog" => Ok(Self::AteTheFrog),
            "inbox_zero" => Ok(Self::InboxZero),
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
