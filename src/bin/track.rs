use error_stack::{Report,Result, ResultExt};
use time_tracker_rust::{error::{AppError, Suggestion}, feature, init};

fn main() -> Result<(), AppError> {
  init::error_reporting();
  init::tracing();

  feature::cli::run().change_context(AppError).attach_printable("Failed to run CLI")?;

  // return Err(Report::from(AppError)).attach(Suggestion("Try running the program again"));

  Ok(())
}
