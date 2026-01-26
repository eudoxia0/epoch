use std::fmt::Display;
use std::fmt::Formatter;

use rusqlite::ToSql;
use rusqlite::types::FromSql;
use rusqlite::types::FromSqlResult;
use rusqlite::types::ToSqlOutput;
use rusqlite::types::ValueRef;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct AchievementId(i32);

impl AchievementId {
    pub fn new(id: i32) -> Self {
        Self(id)
    }
}

impl ToSql for AchievementId {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.0))
    }
}

impl FromSql for AchievementId {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        Ok(AchievementId(FromSql::column_result(value)?))
    }
}

impl Display for AchievementId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
