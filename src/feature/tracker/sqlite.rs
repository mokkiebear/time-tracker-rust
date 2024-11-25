use std::path::PathBuf;

use super::{Reporter, Tracker};

pub struct SqliteTracker {
    database: PathBuf,
    lockfile: PathBuf,
}

// impl Tracker for SqliteTracker {
//     fn start(&mut self) -> error_stack::Result<super::StartupStatus, super::TrackerError> {
//         todo!("lockfile")
//     }

//     fn is_running(&self) -> bool {
//         todo!()
//     }

//     fn stop(&mut self) -> error_stack::Result<(), super::TrackerError> {
//         todo!("lockfile")
//     }

//     fn has_records(&self) -> bool {
//         todo!()
//     }

//     fn records(&self) -> error_stack::Result<impl Iterator<Item = super::TimeRecord>, super::TrackerError> {
//         todo!("database");
//         Ok(())
//     }
// }

// impl Reporter for SqliteTracker {
//     fn total_duration(&self, timespan: super::ReportTimespan) -> error_stack::Result<std::time::Duration, super::ReporterError> {
//         // query the database

//     }
// }