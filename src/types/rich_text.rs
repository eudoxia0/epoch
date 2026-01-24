use pulldown_cmark::Options;
use pulldown_cmark::Parser;
use rusqlite::ToSql;
use rusqlite::types::FromSql;
use rusqlite::types::FromSqlError;
use rusqlite::types::FromSqlResult;
use rusqlite::types::ToSqlOutput;
use rusqlite::types::ValueRef;

use crate::error::AppError;
use crate::error::Fallible;

/// Represents Markdown text.
pub struct RichText(String);

impl RichText {
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

    /// Return the (approximate) number of words in this entry.
    pub fn word_count(&self) -> i32 {
        i32::try_from(self.0.split_whitespace().count()).unwrap_or(i32::MAX)
    }

    /// Render the Markdown to HTML.
    pub fn as_html(&self) -> Fallible<String> {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_SMART_PUNCTUATION);
        let parser = Parser::new_ext(self.as_str(), options);
        let mut html = String::new();
        pulldown_cmark::html::push_html(&mut html, parser);
        Ok(html)
    }
}

impl TryFrom<String> for RichText {
    type Error = AppError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        RichText::new(value)
    }
}

impl ToSql for RichText {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let str = self.as_str();
        Ok(ToSqlOutput::from(str))
    }
}

impl FromSql for RichText {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let string: String = FromSql::column_result(value)?;
        RichText::new(string).map_err(|e| FromSqlError::Other(Box::new(e)))
    }
}
