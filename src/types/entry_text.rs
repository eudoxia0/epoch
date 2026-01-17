use rusqlite::ToSql;
use rusqlite::types::FromSql;
use rusqlite::types::FromSqlError;
use rusqlite::types::FromSqlResult;
use rusqlite::types::ToSqlOutput;
use rusqlite::types::ValueRef;

use crate::error::AppError;
use crate::error::Fallible;

/// Represents the text of an entry.
pub struct EntryText(String);

impl EntryText {
    /// Construct an entry text. Trims the string, and checks it is non-empty.
    pub fn new(text: impl ToString) -> Fallible<Self> {
        let text = text.to_string().trim().to_string();
        if text.is_empty() {
            Err(AppError::new("Entry text must be non-empty."))
        } else {
            Ok(Self(text))
        }
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl TryFrom<String> for EntryText {
    type Error = AppError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        EntryText::new(value)
    }
}

impl ToSql for EntryText {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let str = self.as_str();
        Ok(ToSqlOutput::from(str))
    }
}

impl FromSql for EntryText {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let string: String = FromSql::column_result(value)?;
        EntryText::new(string).map_err(|e| FromSqlError::Other(Box::new(e)))
    }
}
