use assert_cmd::Command;
use predicates::prelude::{PredicateBooleanExt, predicate};

#[test]
fn should_run_cmd_init() {
    let mut cmd = Command::cargo_bin("forgerepo").unwrap();
    cmd.arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains("Initialization"));
}

#[test]
fn should_run_cmd_check() {
    let mut cmd = Command::cargo_bin("forgerepo").unwrap();
    cmd.arg("check")
        .assert()
        .success()
        .stdout(predicate::str::contains("Checking"));
}

#[test]
fn should_run_cmd_help() {
    let stdout_result = predicate::str::contains("Usage: forgerepo <COMMAND>")
        .and(predicate::str::contains("init"))
        .and(predicate::str::contains("check"))
        .and(predicate::str::contains("help"))
        .and(predicate::str::contains("-h, --help"))
        .and(predicate::str::contains("-V, --version"));

    let mut cmd = Command::cargo_bin("forgerepo").unwrap();
    cmd.arg("--help").assert().success().stdout(stdout_result);
}

#[test]
fn should_run_cmd_version() {
    let mut cmd = Command::cargo_bin("forgerepo").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^forgerepo \d+\.\d+\.\d+\n$").unwrap());
}

#[test]
fn should_run_cmd_unknown() {
    let mut cmd = Command::cargo_bin("forgerepo").unwrap();
    cmd.arg("unknown")
        .assert()
        .failure()
        .stderr(predicate::str::contains("unrecognized subcommand"));
}

#[test]
fn should_run_cmd_no_args() {
    let mut cmd = Command::cargo_bin("forgerepo").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage: forgerepo <COMMAND>"));
}
