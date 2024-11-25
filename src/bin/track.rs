use error_stack::{Result, ResultExt};
use time_tracker_rust::{error::{AppError}, feature, init};

fn main() -> Result<(), AppError> {
  init::error_reporting();
  init::tracing();

  feature::cli::run().change_context(AppError).attach_printable("Failed to run CLI")?;

  // return Err(Report::from(AppError)).attach(Suggestion("Try running the program again"));

  Ok(())
}
