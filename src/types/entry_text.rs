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
