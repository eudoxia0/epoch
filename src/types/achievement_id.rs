use rusqlite::ToSql;
use rusqlite::types::FromSql;
use rusqlite::types::FromSqlResult;
use rusqlite::types::ToSqlOutput;
use rusqlite::types::ValueRef;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct AchievementId(i32);

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
