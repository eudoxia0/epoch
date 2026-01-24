use std::collections::HashSet;

use askama::Template;
use axum::Router;
use axum::extract::State;
use axum::response::Html;
use axum::routing::get;
use chrono::Datelike;
use chrono::NaiveDate;
use http::StatusCode;
use rusqlite::Connection;
use rusqlite::Transaction;
use rusqlite::params;
use tokio::sync::MutexGuard;

use crate::db::DayEntry;
use crate::error::Fallible;
use crate::server::ServerState;
use crate::types::Date;

pub struct AllView {}

impl AllView {
    pub fn route(router: Router<ServerState>) -> Router<ServerState> {
        router.route(Self::url(), get(Self::handler))
    }

    pub fn url() -> &'static str {
        "/all"
    }

    async fn handler(State(state): State<ServerState>) -> Fallible<(StatusCode, Html<String>)> {
        let mut db: MutexGuard<'_, Connection> = state.db.lock().await;
        let tx: Transaction<'_> = db.transaction()?;

        // Get all entries and collect their dates into a set for quick lookup
        let entries = list_all_day_entries(&tx)?;
        let entry_dates: HashSet<String> = entries.iter().map(|e| e.date.to_string()).collect();

        // Determine year range
        let current_year = Date::today().year();
        let earliest_year = entries
            .first()
            .map(|e| e.date.year())
            .unwrap_or(current_year);

        // Build the calendar data
        let years: Vec<YearData> = (earliest_year..=current_year)
            .rev()
            .map(|year| build_year_data(year, &entry_dates))
            .collect();

        let tmpl = AllIndexTemplate {
            title: "All Entries",
            years,
        };
        Ok((StatusCode::OK, Html(tmpl.render()?)))
    }
}

#[derive(Template)]
#[template(path = "all_index.jinja2")]
struct AllIndexTemplate<'a> {
    title: &'a str,
    years: Vec<YearData>,
}

struct YearData {
    year: i32,
    months: Vec<MonthData>,
}

struct MonthData {
    name: &'static str,
    first_weekday: u32,
    days: Vec<DayData>,
}

struct DayData {
    day: u32,
    url: String,
    has_entry: bool,
}

fn build_year_data(year: i32, entry_dates: &HashSet<String>) -> YearData {
    let today = Date::today();
    let current_year = today.year();
    let current_month = today.month();

    // For the current year, only show months up to the current month
    let max_month = if year == current_year {
        current_month
    } else {
        12
    };

    let months: Vec<MonthData> = (1..=max_month)
        .map(|month| build_month_data(year, month, entry_dates))
        .collect();

    YearData { year, months }
}

fn build_month_data(year: i32, month: u32, entry_dates: &HashSet<String>) -> MonthData {
    let month_name = chrono::Month::try_from(month as u8)
        .map(|m| m.name())
        .unwrap_or("Unknown");

    // Get the first day of the month
    let first_day = NaiveDate::from_ymd_opt(year, month, 1);
    let first_day = match first_day {
        Some(d) => d,
        None => {
            return MonthData {
                name: "Invalid",
                first_weekday: 0,
                days: Vec::new(),
            };
        }
    };

    // Get the number of days to show in the month
    let today = Date::today();
    let max_day = if year == today.year() && month == today.month() {
        // For the current month, only show days up to today
        today.day()
    } else {
        days_in_month(year, month)
    };

    // Get the weekday of the first day (0 = Monday, 6 = Sunday in chrono)
    // We want Sunday = 0, so adjust
    let first_weekday = first_day.weekday().num_days_from_sunday();

    let days: Vec<DayData> = (1..=max_day)
        .map(|day| {
            let date_str = format!("{:04}-{:02}-{:02}", year, month, day);
            let has_entry = entry_dates.contains(&date_str);
            let url = format!("/day/{}", date_str);
            DayData {
                day,
                url,
                has_entry,
            }
        })
        .collect();

    MonthData {
        name: month_name,
        first_weekday,
        days,
    }
}

fn days_in_month(year: i32, month: u32) -> u32 {
    // Get the first day of the next month, then subtract one day
    let next_month = if month == 12 { 1 } else { month + 1 };
    let next_year = if month == 12 { year + 1 } else { year };

    NaiveDate::from_ymd_opt(next_year, next_month, 1)
        .and_then(|d| d.pred_opt())
        .map(|d| d.day())
        .unwrap_or(30)
}

/// List all day entries in the database, sorted by ascending date.
fn list_all_day_entries(tx: &Transaction<'_>) -> Fallible<Vec<DayEntry>> {
    let sql = "
        select
            date,
            content
        from
            day_entries
        order by
            date asc;
    ";
    let mut stmt = tx.prepare(sql)?;
    let rows = stmt.query_map(params![], |row| {
        Ok(DayEntry {
            date: row.get(0)?,
            content: row.get(1)?,
        })
    })?;
    let mut entries = Vec::new();
    for entry in rows {
        entries.push(entry?);
    }
    Ok(entries)
}
