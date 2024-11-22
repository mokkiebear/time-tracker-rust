use chrono::Utc;
use serde::{Deserialize, Serialize};

mod flatfile;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct StartTime(chrono::DateTime<Utc>);

impl StartTime {
  pub fn now() -> Self {
    Self(Utc::now())
  }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct EndTime(chrono::DateTime<Utc>);

impl EndTime {
  pub fn now() -> Self {
    Self(Utc::now())
  }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TimeRecord {
  pub start: StartTime,
  pub end: EndTime,
}