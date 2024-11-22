use error_stack::Result;
use clap::{Parser, Subcommand};

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
  #[command(subcommand)]
  pub command: Command
}

pub fn run() -> Result<(), CLIError> {
  let args = Cli::parse();

  match args.command {
      Command::Start => todo!(),
      Command::Stop => todo!(),
      Command::Report => todo!(),
  }

  Ok(())
}