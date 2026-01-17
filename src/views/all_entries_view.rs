use std::collections::HashSet;

use axum::Router;
use axum::extract::State;
use axum::response::Html;
use axum::routing::get;
use chrono::Datelike;
use chrono::NaiveDate;
use http::StatusCode;
use maud::Markup;
use maud::html;
use rusqlite::Connection;
use rusqlite::Transaction;
use tokio::sync::MutexGuard;

use crate::db::list_all_day_entries;
use crate::error::Fallible;
use crate::server::ServerState;
use crate::templates::page_template;
use crate::types::Date;

pub struct AllEntriesView {}

impl AllEntriesView {
    pub fn route(router: Router<ServerState>) -> Router<ServerState> {
        router.route(Self::url(), get(Self::handler))
    }

    pub fn url() -> &'static str {
        "/all"
    }

    async fn handler(
        State(state): State<ServerState>,
    ) -> Fallible<(StatusCode, Html<String>)> {
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

        // Build the calendar HTML
        let body: Markup = html! {
            .all-entries {
                .all-entries-header {
                    h1 { "All Entries" }
                }
                .all-entries-body {
                    @for year in (earliest_year..=current_year).rev() {
                        (render_year(year, &entry_dates))
                    }
                }
            }
        };

        let html = page_template("All Entries", body);
        Ok((StatusCode::OK, Html(html.into_string())))
    }
}

fn render_year(year: i32, entry_dates: &HashSet<String>) -> Markup {
    html! {
        .calendar-year {
            h2.year-title { (year) }
            .months-grid {
                @for month in 1..=12u32 {
                    (render_month(year, month, entry_dates))
                }
            }
        }
    }
}

fn render_month(year: i32, month: u32, entry_dates: &HashSet<String>) -> Markup {
    let month_name = chrono::Month::try_from(month as u8)
        .map(|m| m.name())
        .unwrap_or("Unknown");

    // Get the first day of the month
    let first_day = NaiveDate::from_ymd_opt(year, month, 1);
    let first_day = match first_day {
        Some(d) => d,
        None => return html! { .calendar-month { p { "Invalid month" } } },
    };

    // Get the number of days in the month
    let days_in_month = days_in_month(year, month);

    // Get the weekday of the first day (0 = Monday, 6 = Sunday in chrono)
    // We want Sunday = 0, so adjust
    let first_weekday = first_day.weekday().num_days_from_sunday();

    html! {
        .calendar-month {
            h3.month-title { (month_name) }
            .calendar-grid {
                // Weekday headers
                .calendar-header { "Su" }
                .calendar-header { "Mo" }
                .calendar-header { "Tu" }
                .calendar-header { "We" }
                .calendar-header { "Th" }
                .calendar-header { "Fr" }
                .calendar-header { "Sa" }

                // Empty cells before the first day
                @for _ in 0..first_weekday {
                    .calendar-day.empty {}
                }

                // Days of the month
                @for day in 1..=days_in_month {
                    @let date_str = format!("{:04}-{:02}-{:02}", year, month, day);
                    @let has_entry = entry_dates.contains(&date_str);
                    @let url = format!("/day/{}", date_str);
                    .calendar-day {
                        a href=(url) class=@if !has_entry { "no-entry" } {
                            (day)
                        }
                    }
                }
            }
        }
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
