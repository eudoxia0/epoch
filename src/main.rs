use std::process::ExitCode;

use crate::cli::entrypoint;

mod assets;
mod cli;
mod config;
mod db;
mod error;
mod server;
mod templates;
mod types;
mod views;

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    entrypoint().await
}
