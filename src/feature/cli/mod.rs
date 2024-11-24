use std::path::PathBuf;

use clap::{Parser, Subcommand};
use error_stack::{Result, ResultExt};

use crate::{error::Suggestion, feature::tracker::FlatFileTracker};

use super::tracker::StartupStatus;
use crate::feature::tracker::Tracker;

#[derive(Debug, thiserror::Error)]
#[error("CLI error occurred")]
pub struct CLIError;

#[derive(Debug, Clone, Copy, Subcommand)]
pub enum Command {
    /// Start tracking time
    Start,
    Stop,
    Report,
}

#[derive(Parser, Debug)]
#[command(version, about, arg_required_else_help(true))]
pub struct Cli {
    /// Path to the database directory
    #[arg(short = 'd', long)]
    pub db_dir: Option<PathBuf>,

    /// Path to the lockfile
    #[arg(short = 'l', long)]
    pub lockfile: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Command,
}

pub fn run() -> Result<(), CLIError> {
    let args = Cli::parse();

    let db_dir = flatfile_db_dir(&args)?;
    let lockfile = lockfile_path(&args)?;

    let mut tracker = FlatFileTracker::new(db_dir, lockfile);

    match args.command {
        Command::Start => match tracker.start() {
            Ok(StartupStatus::Started) => println!("Tracker started"),
            Ok(StartupStatus::Running) => println!("Tracker already running"),
            Err(e) => return Err(e).change_context(CLIError),
        },
        Command::Stop => tracker.stop().change_context(CLIError).attach_printable("failed to stop tracker")?,
        Command::Report => todo!(),
    }

    Ok(())
}

fn flatfile_db_dir(args: &Cli) -> Result<PathBuf, CLIError> {
    match &args.db_dir {
        Some(db_dir) => Ok(db_dir.clone()),
        None => {
            let mut db_path: PathBuf = dirs::data_dir()
                .ok_or(CLIError)
                .attach_printable("failed to get data dir")
                .attach(Suggestion("use -d flag to specify the database directory"))?;

            db_path.push("track");

            std::fs::create_dir_all(&db_path)
                .change_context(CLIError)
                .attach_printable("failed to create database directory")?;

            db_path.push("records.json");

            Ok(db_path)
        }
    }
}

fn lockfile_path(args: &Cli) -> Result<PathBuf, CLIError> {
    match &args.lockfile {
        Some(lockfile) => Ok(lockfile.clone()),
        None => {
            let mut lockfile: PathBuf = dirs::cache_dir()
                .ok_or(CLIError)
                .attach_printable("failed to get cache dir")
                .attach(Suggestion("use -l flag to specify the lockfile directory"))?;

            lockfile.push("track");

            std::fs::create_dir_all(&lockfile)
                .change_context(CLIError)
                .attach_printable("failed to create 'track' cache directory")?;

            lockfile.push("track.lock");

            Ok(lockfile)
        }
    }
}
