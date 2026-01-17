mod all;
mod css;
mod day;
mod favicon;
mod index;
mod not_found_handler;
mod search;
mod stats;

pub use all::AllView;
pub use css::CssView;
pub use day::*;
pub use favicon::FaviconView;
pub use index::IndexView;
pub use not_found_handler::not_found_handler;
pub use search::*;
pub use stats::StatsView;
