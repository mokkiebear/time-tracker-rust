mod flatfile;
mod reporter;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use error_stack::Result;

pub use flatfile::FlatFileTracker;
pub use reporter::{Reporter, ReporterError,ReportTimespan};

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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum StartupStatus {
    /// Time tracker started
    Started,
    /// Time tracker was already running
    Running,
}

#[derive(Debug, thiserror::Error)]
#[error("tracker error")]
pub struct TrackerError;

pub trait Tracker {
  fn start(&mut self) -> Result<StartupStatus, TrackerError>;

  fn is_running(&self) -> bool;

  fn stop(&mut self) -> Result<(), TrackerError>;

  fn has_records(&self) -> bool;

  fn records(&self) -> Result<impl Iterator<Item = TimeRecord>, TrackerError>;
}

#[cfg(test)]
pub mod tlib {
  use super::*;

  #[derive(Debug, Default)]
  pub struct FakeTracker {
    tracking: Option<StartTime>,
    records: Vec<TimeRecord>,
  }

  impl Tracker for FakeTracker {
    fn start(&mut self) -> Result<StartupStatus, TrackerError> {
      if self.tracking.is_some() {
        Ok(StartupStatus::Running)
      } else {
        self.tracking = Some(StartTime::now());
        Ok(StartupStatus::Started)
      }
    }

    fn is_running(&self) -> bool {
      self.tracking.is_some()
    }

    fn stop(&mut self) -> Result<(), TrackerError> {
      let start_time = self.tracking.take().unwrap();
      let end_time = EndTime::now();

      let record = TimeRecord { start: start_time, end: end_time };
      self.records.push(record);
      Ok(())
    }

    fn has_records(&self) -> bool {
      !self.records.is_empty()
    }

    fn records(&self) -> Result<impl Iterator<Item = TimeRecord>, TrackerError> {
      Ok(self.records.iter().cloned())
    }
  }
}