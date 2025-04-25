mod utils;

use std::process::Command;

use crate::utils::command::get_air_bin;
use crate::utils::command::run;

#[test]
fn test_help() {
    insta::assert_snapshot!(run(&mut Command::new(get_air_bin())));
    insta::assert_snapshot!(run(Command::new(get_air_bin()).arg("help")));
    insta::assert_snapshot!(run(Command::new(get_air_bin()).arg("--help")));
    insta::assert_snapshot!(run(Command::new(get_air_bin()).arg("-h")));
}

#[test]
fn test_format_help() {
    insta::assert_snapshot!(run(Command::new(get_air_bin()).arg("format")));
    insta::assert_snapshot!(run(Command::new(get_air_bin()).arg("format").arg("--help")));
    insta::assert_snapshot!(run(Command::new(get_air_bin()).arg("format").arg("-h")));
}
