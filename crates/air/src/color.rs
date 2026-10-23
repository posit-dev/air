use std::ffi::OsStr;
use std::io::IsTerminal;

/// Decides whether the CLI colors its output
///
/// We use colored output in three places:
/// - Level labels in `tracing-subscriber`, like `ERROR` , which shows up in red
/// - Usage of the `colored` crate
/// - Diagnostics printed through `biome_console::EnvConsole`
///
/// To avoid potential differences between the tools we do a one time resolution
/// of user overrides and `NO_COLOR` ourselves, then forcibly set the option for
/// each of them.
pub(crate) fn init_color(no_color: bool) -> bool {
    let color =
        // User override via `--no-color`
        !no_color
        // User override via `NO_COLOR` env var
        && !no_color_env_var_is_set()
        // Only emit colors if the target is a terminal. We only emit colored
        // output to stderr, so we don't need to check stdout.
        && std::io::stderr().is_terminal();

    // Apply our decision globally to `colored`
    colored::control::set_override(color);

    // Return our decision so we can apply it to `tracing-subscriber` and the
    // `biome_console::EnvConsole`
    color
}

/// Is `NO_COLOR` set to a non-empty, non-zero value?
fn no_color_env_var_is_set() -> bool {
    env_var_is_set(std::env::var_os("NO_COLOR").as_deref())
}

fn env_var_is_set(value: Option<&OsStr>) -> bool {
    value.is_some_and(|value| !value.is_empty() && value != "0")
}

#[cfg(test)]
mod test {
    use std::ffi::OsStr;

    use crate::color::env_var_is_set;

    #[test]
    fn test_env_var_is_set() {
        assert!(!env_var_is_set(None));
        assert!(!env_var_is_set(Some(OsStr::new(""))));
        assert!(!env_var_is_set(Some(OsStr::new("0"))));
        assert!(env_var_is_set(Some(OsStr::new("1"))));
        assert!(env_var_is_set(Some(OsStr::new("true"))));
    }
}
