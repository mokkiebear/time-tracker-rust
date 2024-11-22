use assert_cmd::Command;
use testresult::TestResult;

#[test]
fn status_code_is_error_if_no_command_specified() -> TestResult {
    Command::cargo_bin("track")?.assert().failure();

    Ok(())
}

#[test]
fn start_command_starts_tracking() -> TestResult {
    Command::cargo_bin("track")?.arg("start").assert().success();

    todo!("Implement the start command");

    #[allow(unreachable_code)]
    Ok(())
}

#[test]
fn stop_command_stops_tracking() -> TestResult {
    Command::cargo_bin("track")?.arg("start").assert().success();
    Command::cargo_bin("track")?.arg("stop").assert().success();

    todo!("Implement the stop command");

    #[allow(unreachable_code)]
    Ok(())
}

#[test]
fn report_command_generates_report() -> TestResult {
    Command::cargo_bin("track")?.arg("start").assert().success();
    Command::cargo_bin("track")?.arg("stop").assert().success();

    Command::cargo_bin("track")?
        .arg("report")
        .assert()
        .stdout("00:00:00\n");

    todo!("Implement the stop command");

    #[allow(unreachable_code)]
    Ok(())
}
