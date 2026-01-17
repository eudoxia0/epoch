use std::fmt::Display;
use std::fmt::Formatter;

use chrono::Datelike;
use chrono::Local;
use chrono::Month;
use chrono::NaiveDate;
use chrono::Weekday;
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

    pub fn year(self) -> i32 {
        self.0.year()
    }

    fn month_name(self) -> &'static str {
        Month::try_from(self.0.month() as u8).unwrap().name()
    }

    pub fn weekday(self) -> &'static str {
        match self.0.weekday() {
            Weekday::Sun => "Sunday",
            Weekday::Mon => "Monday",
            Weekday::Tue => "Tuesday",
            Weekday::Wed => "Wednesday",
            Weekday::Thu => "Thursday",
            Weekday::Fri => "Friday",
            Weekday::Sat => "Saturday",
        }
    }

    /// Render the date like "17 January, 2026".
    pub fn humanize(self) -> String {
        let day = self.0.day();
        let month = self.month_name();
        let year = self.year();
        format!("{day} {month}, {year}")
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
