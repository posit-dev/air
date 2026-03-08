use std::process::Command;

use crate::helpers::CommandExt;
use crate::helpers::binary_path;

#[test]
fn test_generate_completions_bash() {
    insta::assert_snapshot!(
        Command::new(binary_path())
            .arg("completion")
            .arg("bash")
            .run()
            .normalize_os_executable_name()
    );
}

#[test]
fn test_generate_completions_zsh() {
    insta::assert_snapshot!(
        Command::new(binary_path())
            .arg("completion")
            .arg("zsh")
            .run()
            .normalize_os_executable_name()
    );
}

#[test]
fn test_generate_completions_fish() {
    insta::assert_snapshot!(
        Command::new(binary_path())
            .arg("completion")
            .arg("fish")
            .run()
            .normalize_os_executable_name()
    );
}
