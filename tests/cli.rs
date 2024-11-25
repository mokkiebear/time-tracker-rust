use assert_cmd::Command;
use assert_fs::{fixture::ChildPath, prelude::PathChild, TempDir};
use testresult::TestResult;

fn tracking_paths() -> (TempDir, ChildPath, ChildPath) {
    let temp = TempDir::new().unwrap();
    let db = temp.child("db.json");
    let lockfile = temp.child("lockfile");

    (temp, db, lockfile)
}

fn start_tracking(db: &ChildPath, lockfile: &ChildPath) -> Result<(), testresult::TestError> {
    Command::cargo_bin("track")?
        .arg("--db-dir")
        .arg(db.to_path_buf())
        .arg("--lockfile")
        .arg(lockfile.to_path_buf())
        .arg("start")
        .assert()
        .success();
    Ok(())
}

fn stop_tracking(db: &ChildPath, lockfile: &ChildPath) -> Result<(), testresult::TestError> {
    Command::cargo_bin("track")?
        .arg("--db-dir")
        .arg(db.to_path_buf())
        .arg("--lockfile")
        .arg(lockfile.to_path_buf())
        .arg("stop")
        .assert()
        .success();
    Ok(())
}

#[test]
fn status_code_is_error_if_no_command_specified() -> TestResult {
    Command::cargo_bin("track")?.assert().failure();

    Ok(())
}

#[test]
fn start_command_starts_tracking() -> TestResult {
    // track --db-dir PATH --lockfile PATH start

    let (temp, db, lockfile) = tracking_paths();

    start_tracking(&db, &lockfile)?;

    assert!(lockfile.to_path_buf().exists());

    Ok(())
}

#[test]
fn stop_command_stops_tracking() -> TestResult {
    // track --db-dir PATH --lockfile PATH stop

    let (temp, db, lockfile) = tracking_paths();

    start_tracking(&db, &lockfile)?;

    assert!(lockfile.to_path_buf().exists());

    stop_tracking(&db, &lockfile)?;

    assert!(!lockfile.to_path_buf().exists());

    Ok(())
}

#[test]
fn report_command_generates_report() -> TestResult {
    let (temp, db, lockfile) = tracking_paths();

    start_tracking(&db, &lockfile)?;
    stop_tracking(&db, &lockfile)?;

    Command::cargo_bin("track")?
        .arg("--db-dir")
        .arg(db.to_path_buf())
        .arg("--lockfile")
        .arg(lockfile.to_path_buf())
        .arg("report")
        .assert()
        .stdout("00:00:00\n");

    Ok(())
}
