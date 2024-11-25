// flat file tracker
// 2 files:
// - "lockfile": tracker is running
// - "database.json": all records

use error_stack::{Result, ResultExt};
use serde::{Deserialize, Serialize};
use std::{
    fs::OpenOptions,
    io::{Read, Write},
    path::PathBuf,
};

use super::{EndTime, Reporter, StartTime, StartupStatus, TimeRecord, Tracker, TrackerError};

#[derive(Debug, thiserror::Error)]
#[error("file system tracker error")]
pub struct FlatFileTrackerError;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct LockfileData {
    start_time: StartTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct FlatFileDatabase {
    records: Vec<TimeRecord>,
}

impl FlatFileDatabase {
    pub fn push(&mut self, value: TimeRecord) {
        self.records.push(value)
    }
}

pub struct FlatFileTracker {
    database: PathBuf,
    lockfile: PathBuf,
}

impl Reporter for FlatFileTracker {}

impl FlatFileTracker {
    pub fn new<D, L>(database: D, lockfile: L) -> Self
    where
        D: Into<PathBuf>,
        L: Into<PathBuf>,
    {
        let database = database.into();
        let lockfile = lockfile.into();

        FlatFileTracker { database, lockfile }
    }
}

impl Tracker for FlatFileTracker {
    fn start(&mut self) -> Result<StartupStatus, TrackerError> {
        if self.lockfile.exists() {
            return Ok(StartupStatus::Running);
        }

        let lockfile_data = {
            let start_time = StartTime::now();
            let data = LockfileData { start_time };

            serde_json::to_string(&data)
                .change_context(TrackerError)
                .attach_printable("unable to serialize lockfile data when starting tracker")?
        };

        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&self.lockfile)
            .change_context(TrackerError)
            .attach_printable("unable to create new lockfile when starting tracker")?
            .write_all(lockfile_data.as_bytes())
            .change_context(TrackerError)
            .attach_printable("unable to write lockfile data when starting tracker")?;

        Ok(StartupStatus::Started)
    }

    fn is_running(&self) -> bool {
        self.lockfile.exists()
    }

    fn stop(&mut self) -> Result<(), TrackerError> {
        let start = read_lockfile(&self.lockfile)?.start_time;
        let end = EndTime::now();

        let record = TimeRecord { start, end };

        let mut db = load_database(&self.database)?;
        db.push(record);
        save_database(&self.database, &db)?;

        std::fs::remove_file(&self.lockfile)
            .change_context(TrackerError)
            .attach_printable("unable to remove lockfile when stopping tracker")?;

        Ok(())
    }

    fn has_records(&self) -> bool {
        let db = load_database(&self.database).unwrap();
        !db.records.is_empty()
    }

    fn records(&self) -> Result<impl Iterator<Item = TimeRecord>, TrackerError> {
        let db = load_database(&self.database)?;
        Ok(db.records.into_iter())
    }
}

fn save_database<P>(path: P, db: &FlatFileDatabase) -> Result<(), TrackerError>
where
    P: AsRef<std::path::Path>,
{
    let db = serde_json::to_string(&db)
        .change_context(TrackerError)
        .attach_printable("failed to serialize database")?;

    OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(false)
        .open(path.as_ref())
        .change_context(TrackerError)
        .attach_printable("unable to open database when writing")?
        .write_all(&db.as_bytes())
        .change_context(TrackerError)
        .attach_printable("unable to write database when writing")?;

    Ok(())
}

fn load_database<P>(database: P) -> Result<FlatFileDatabase, TrackerError>
where
    P: AsRef<std::path::Path>,
{
    let mut db_buf = String::default();
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(database.as_ref())
        .change_context(TrackerError)
        .attach_printable("unable to open database when reading")?
        .read_to_string(&mut db_buf)
        .change_context(TrackerError)
        .attach_printable("unable to read database when reading")?;

    if db_buf.is_empty() {
        return Ok(FlatFileDatabase::default());
    } else {
        Ok(serde_json::from_str(&db_buf)
            .change_context(TrackerError)
            .attach_printable("unable to deserialize database data when reading")?)
    }
}

fn read_lockfile<P>(lockfile: P) -> Result<LockfileData, TrackerError>
where
    P: AsRef<std::path::Path>,
{
    let file = OpenOptions::new()
        .read(true)
        .open(lockfile.as_ref())
        .change_context(TrackerError)
        .attach_printable("unable to open lockfile when reading")?;

    Ok(serde_json::from_reader(file)
        .change_context(TrackerError)
        .attach_printable("unable to deserialize lockfile data when reading")?)
}

#[cfg(test)]
mod tests {
    use assert_fs::{fixture::ChildPath, prelude::PathChild, TempDir};

    use super::*;

    fn tracking_paths() -> (TempDir, ChildPath, ChildPath) {
        let temp = TempDir::new().unwrap();
        let db = temp.child("db.json");
        let lockfile = temp.child("lockfile");

        (temp, db, lockfile)
    }

    fn new_tracker(db: &ChildPath, lockfile: &ChildPath) -> FlatFileTracker {
        FlatFileTracker::new(db.to_path_buf(), lockfile.to_path_buf())
    }

    #[test]
    fn starts_tracking_with_default_tracker() {
        let (_temp, db, lockfile) = tracking_paths();
        // Given a new tracker
        let mut tracker = new_tracker(&db, &lockfile);

        // When the tracker is started
        tracker.start().unwrap();

        // Then the tracker is running
        assert!(tracker.is_running());
    }

    #[test]
    fn is_running_returns_false_after_stopping_tracker() {
        let (_temp, db, lockfile) = tracking_paths();

        // Given a new tracker
        let mut tracker = new_tracker(&db, &lockfile);
        // When the tracker is started
        tracker.start().unwrap();
        tracker.stop().unwrap();

        // Then the tracker is running
        assert!(!tracker.is_running());
    }

    #[test]
    fn time_record_created_when_tracking_stops() {
        let (_temp, db, lockfile) = tracking_paths();

        // Given a new tracker
        let mut tracker = new_tracker(&db, &lockfile);
        // When the tracker is started
        tracker.start().unwrap();

        std::thread::sleep(std::time::Duration::from_secs(1));

        tracker.stop().unwrap();

        // Then a record is saved
        assert!(tracker.records().unwrap().next().is_some());
    }

    #[test]
    fn initial_start_returns_started_state() {
        let (_temp, db, lockfile) = tracking_paths();

        // Given a new tracker
        let mut tracker = new_tracker(&db, &lockfile);
        // When the tracker is started
        let started = tracker.start().unwrap();

        std::thread::sleep(std::time::Duration::from_secs(1));

        // Then the "already running" state is returned
        assert_eq!(started, StartupStatus::Started);
    }

    #[test]
    fn multiple_starts_returns_already_running_state() {
        let (_temp, db, lockfile) = tracking_paths();

        // Given a new tracker
        let mut tracker = new_tracker(&db, &lockfile);
        // When the tracker is started
        tracker.start().unwrap();

        std::thread::sleep(std::time::Duration::from_secs(1));

        let started = tracker.start().unwrap();

        // Then the "already running" state is returned
        assert_eq!(started, StartupStatus::Running);
    }
}
