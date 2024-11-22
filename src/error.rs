//! Top-level errors

#[derive(Debug, thiserror::Error)]
#[error("Application error occurred")]
pub struct AppError;

pub struct Suggestion(pub &'static str);