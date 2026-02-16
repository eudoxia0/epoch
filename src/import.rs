use std::fs;
use std::path::PathBuf;

use crate::config::Config;
use crate::error::AppError;
use crate::error::Fallible;
use crate::models::DayEntry;
use crate::models::open_db;
use crate::types::Date;
use crate::types::RichText;

pub fn import_day_entry(path: PathBuf) -> Fallible<()> {
    // Path must point to a file that exists.
    if !path.exists() {
        return Err(AppError::new(format!(
            "File does not exist: {}",
            path.display()
        )));
    }
    if !path.is_file() {
        return Err(AppError::new(format!(
            "Path is not a file: {}",
            path.display()
        )));
    }

    // File must have the extension `.md`.
    match path.extension() {
        Some(ext) if ext == "md" => {}
        _ => return Err(AppError::new("File must have the .md extension.")),
    }

    // File name must be in the format `YYYY-MM-DD`.
    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| AppError::new("Could not read file name."))?;
    let date = Date::try_from(stem.to_string())?;

    // Read file content.
    let content = fs::read_to_string(&path)?;
    let rich_text = RichText::new(content)?;

    // Open database and check if entry exists.
    let config = Config::from_env()?;
    let mut db = open_db(&config.db_path)?;
    let tx = db.transaction()?;

    if DayEntry::get_opt(&tx, date)?.is_some() {
        return Err(AppError::new(format!(
            "An entry for {} already exists. Will not overwrite.",
            date
        )));
    }

    // Create the entry.
    DayEntry::create(&tx, date, &rich_text)?;
    tx.commit()?;

    println!("Imported entry for {}.", date);
    Ok(())
}
