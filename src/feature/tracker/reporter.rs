use std::time::Duration;

use chrono::Utc;
use error_stack::{ResultExt, Result};

use super::Tracker;

#[derive(Debug, Clone, Copy)]
pub enum ReportTimespan {
    Last(Duration),
}

#[derive(Debug, thiserror::Error)]
#[error("reporter error")]
pub struct ReporterError;

pub trait Reporter: Tracker {
    fn total_duration(&self, timespan: ReportTimespan) -> Result<Duration, ReporterError> {
        match timespan {
            ReportTimespan::Last(duration) => {
                let now = Utc::now();
                let target = (now - duration).timestamp_millis();
                let total_ms = self
                    .records()
                    .change_context(ReporterError)
                    .attach_printable("failed to query records")?
                    .filter_map(|rec| {
                        if rec.start.0.timestamp_millis() >= target {
                            Some(rec.end.0.timestamp_millis() - rec.start.0.timestamp_millis())
                        } else {
                            None
                        }
                    })
                    .sum::<i64>();

                Ok(Duration::from_millis(total_ms as u64))
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::feature::tracker::tlib::FakeTracker;

    use super::*;

    impl Reporter for FakeTracker {
      
    }

    #[test]
    fn calculate_correct_duration_when_there_are_no_records() {
        let tracker = FakeTracker::default();

        let duration = tracker
            .total_duration(ReportTimespan::Last(Duration::from_secs(1)))
            .unwrap();

        assert_eq!(duration, Duration::from_millis(0));
    }

    #[test]
    fn calculate_correct_duration_when_there_are_two_records() {
        let mut tracker = FakeTracker::default();

        tracker.start().unwrap();
        std::thread::sleep(Duration::from_millis(10));
        tracker.stop().unwrap();

        tracker.start().unwrap();
        std::thread::sleep(Duration::from_millis(10));
        tracker.stop().unwrap();

        let duration = tracker
            .total_duration(ReportTimespan::Last(Duration::from_secs(1)))
            .unwrap();

        assert!(duration >= Duration::from_millis(20));
    }
}
