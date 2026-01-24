use std::process::ExitCode;

use crate::cli::entrypoint;

mod assets;
mod cli;
mod config;
mod controllers;
mod db;
mod error;
mod server;
mod types;

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    entrypoint().await
}
