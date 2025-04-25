mod utils;

use std::process::Command;

use crate::utils::command::get_air_bin;
use crate::utils::command::run;
use crate::utils::fixtures::path_fixtures;

#[test]
fn test_check_when_no_formatting_is_required() {
    let fixtures = path_fixtures();

    let path = fixtures.join("format").join("formatted.R");
    let path = path.to_str().unwrap();

    insta::assert_snapshot!(run(Command::new(get_air_bin())
        .arg("format")
        .arg(path)
        .arg("--check")));

    insta::assert_snapshot!(run(Command::new(get_air_bin())
        .arg("format")
        .arg(path)
        .arg("--check=interactive")));

    insta::assert_snapshot!(run(Command::new(get_air_bin())
        .arg("format")
        .arg(path)
        .arg("--check=github")));
}

#[test]
fn test_check_output_format() {
    let fixtures = path_fixtures();

    let path1 = fixtures.join("format").join("needs-formatting-1.R");
    let path1 = path1.to_str().unwrap();

    let path2 = fixtures.join("format").join("needs-formatting-2.R");
    let path2 = path2.to_str().unwrap();

    insta::assert_snapshot!(run(Command::new(get_air_bin())
        .arg("format")
        .arg(path1)
        .arg(path2)
        .arg("--check")));

    insta::assert_snapshot!(run(Command::new(get_air_bin())
        .arg("format")
        .arg(path1)
        .arg(path2)
        .arg("--check=interactive")));

    insta::assert_snapshot!(run(Command::new(get_air_bin())
        .arg("format")
        .arg(path1)
        .arg(path2)
        .arg("--check=github")));
}

#[test]
fn test_check_invalid_output_format() {
    insta::assert_snapshot!(run(Command::new(get_air_bin())
        .arg("format")
        .arg("path.R")
        .arg("--check=foo")));
}
