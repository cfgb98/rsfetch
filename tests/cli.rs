//! Integration tests driving the built `rsfetch` binary.
//!
//! These assert structural properties of the output (which lines appear, that
//! color is on/off, exit codes) rather than exact system values, which vary by
//! machine.

use assert_cmd::Command;
use predicates::prelude::*;

fn rsfetch() -> Command {
    Command::cargo_bin("rsfetch").unwrap()
}

#[test]
fn runs_successfully_by_default() {
    rsfetch().assert().success();
}

#[test]
fn fields_flag_limits_output() {
    rsfetch()
        .args(["--fields", "host", "--logo", "none", "--color", "never"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Host"))
        .stdout(predicate::str::contains("Memory").not());
}

#[test]
fn color_never_emits_no_ansi() {
    rsfetch()
        .args(["--color", "never", "--logo", "none"])
        .assert()
        .success()
        .stdout(predicate::str::contains('\u{1b}').not());
}

#[test]
fn color_always_emits_ansi() {
    rsfetch()
        .args(["--color", "always", "--logo", "none", "--fields", "os"])
        .assert()
        .success()
        .stdout(predicate::str::contains('\u{1b}'));
}

#[test]
fn unknown_field_fails() {
    rsfetch()
        .args(["--fields", "bogus"])
        .assert()
        .failure();
}

#[test]
fn unknown_logo_fails_with_message() {
    rsfetch()
        .args(["--logo", "definitely-not-a-logo"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("unknown logo"));
}

#[test]
fn missing_explicit_config_fails() {
    rsfetch()
        .args(["--config", "/no/such/rsfetch/config.toml"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("could not read config"));
}

#[test]
fn separator_flag_is_applied() {
    rsfetch()
        .args(["--logo", "none", "--color", "never", "--fields", "os", "--separator", "::"])
        .assert()
        .success()
        .stdout(predicate::str::contains("::"));
}

#[test]
fn help_lists_options() {
    rsfetch()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--fields"))
        .stdout(predicate::str::contains("--logo"));
}
