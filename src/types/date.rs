use std::fmt::Display;
use std::fmt::Formatter;

use chrono::Local;
use chrono::NaiveDate;
use rusqlite::ToSql;
use rusqlite::types::FromSql;
use rusqlite::types::FromSqlError;
use rusqlite::types::FromSqlResult;
use rusqlite::types::ToSqlOutput;
use rusqlite::types::ValueRef;
use serde::Deserialize;

use crate::error::AppError;

/// A naive date.
#[derive(Clone, Copy, Deserialize)]
#[serde(try_from = "String")]
pub struct Date(NaiveDate);

impl Date {
    /// Today's date.
    pub fn today() -> Self {
        Self(Local::now().naive_local().date())
    }

    /// Return the date one day before this date.
    pub fn previous(self) -> Self {
        Self(self.0.pred_opt().unwrap_or(self.0))
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%d"))
    }
}

impl TryFrom<String> for Date {
    type Error = AppError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let date = NaiveDate::parse_from_str(&value, "%Y-%m-%d")
            .map_err(|_| AppError::new(format!("invalid date: {}", value)))?;
        Ok(Date(date))
    }
}

impl ToSql for Date {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let str = self.to_string();
        Ok(ToSqlOutput::from(str))
    }
}

impl FromSql for Date {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let string: String = FromSql::column_result(value)?;
        Date::try_from(string).map_err(|e| FromSqlError::Other(Box::new(e)))
    }
}
