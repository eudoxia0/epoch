use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;

use crate::error::Fallible;
use crate::import::import_day_entry;
use crate::server::start_server;

#[derive(Parser)]
#[command(version, about, long_about = None)]
enum Command {
    /// Start a server.
    Serve,
    /// Import a day entry.
    Import { path: PathBuf },
}

/// The program entrypoint.
pub async fn entrypoint() -> ExitCode {
    env_logger::init();
    let cmd: Command = Command::parse();
    let res: Fallible<()> = match cmd {
        Command::Serve => start_server().await,
        Command::Import { path } => import_day_entry(path),
    };
    match res {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}
