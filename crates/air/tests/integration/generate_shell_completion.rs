use std::process::Command;

use crate::helpers::CommandExt;
use crate::helpers::binary_path;

#[test]
fn test_completions_bash() {
    insta::assert_snapshot!(
        Command::new(binary_path())
            .arg("generate-shell-completion")
            .arg("bash")
            .run()
            .normalize_os_executable_name()
    );
}

#[test]
fn test_completions_elvish() {
    insta::assert_snapshot!(
        Command::new(binary_path())
            .arg("generate-shell-completion")
            .arg("elvish")
            .run()
            .normalize_os_executable_name()
    );
}

#[test]
fn test_completions_fish() {
    insta::assert_snapshot!(
        Command::new(binary_path())
            .arg("generate-shell-completion")
            .arg("fish")
            .run()
            .normalize_os_executable_name()
    );
}

#[test]
fn test_completions_powershell() {
    insta::assert_snapshot!(
        Command::new(binary_path())
            .arg("generate-shell-completion")
            .arg("powershell")
            .run()
            .normalize_os_executable_name()
    );
}

#[test]
fn test_completions_zsh() {
    insta::assert_snapshot!(
        Command::new(binary_path())
            .arg("generate-shell-completion")
            .arg("zsh")
            .run()
            .normalize_os_executable_name()
    );
}
