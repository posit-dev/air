use std::path::Path;

use workspace::discovery;
use workspace::discovery::DiscoveredSettings;
use workspace::resolve::PathResolver;
use workspace::settings::Settings;

use crate::ExitStatus;
use crate::args::FormatCommand;

mod paths;
mod stdin;

#[derive(Copy, Clone, Debug)]
enum FormatMode {
    Write,
    Check,
}

#[derive(Copy, Clone, Debug)]
enum ConfigurationMode {
    Discover,
    Disabled,
}

pub(crate) fn format(command: FormatCommand) -> anyhow::Result<ExitStatus> {
    if let Some(status) = check_argument_consistency(&command) {
        return Ok(status);
    }

    let mode = FormatMode::from_command(&command);
    let configuration = ConfigurationMode::from_command(&command);

    let (exclude, include) = if command.force {
        (discovery::Exclude::Nothing, discovery::Include::Everything)
    } else {
        (discovery::Exclude::Matched, discovery::Include::Matched)
    };

    match command.stdin_file_path {
        Some(path) => stdin::format(path, mode, configuration, exclude, include),
        None => paths::format(command.paths, mode, configuration, exclude, include),
    }
}

fn resolve_settings<P: AsRef<Path>>(
    paths: &[P],
    configuration: ConfigurationMode,
) -> anyhow::Result<(PathResolver<Settings>, Settings)> {
    let mut resolver = PathResolver::new();

    if let ConfigurationMode::Discover = configuration {
        for DiscoveredSettings {
            directory,
            settings,
        } in discovery::discover_settings(paths)?
        {
            resolver.add(&directory, settings);
        }

        let default_settings = discovery::discover_user_settings()?.unwrap_or_default();
        return Ok((resolver, default_settings));
    }

    Ok((resolver, Settings::default()))
}

fn check_argument_consistency(command: &FormatCommand) -> Option<ExitStatus> {
    if command.stdin_file_path.is_some() && !command.paths.is_empty() {
        tracing::error!(
            "Can't supply paths when reading from stdin: {paths}",
            paths = command
                .paths
                .iter()
                .map(|path| format!("'{path}'", path = path.display()))
                .collect::<Vec<String>>()
                .join(",")
        );
        return Some(ExitStatus::Error);
    }

    None
}

impl FormatMode {
    fn from_command(command: &FormatCommand) -> Self {
        if command.check {
            FormatMode::Check
        } else {
            FormatMode::Write
        }
    }
}

impl ConfigurationMode {
    fn from_command(command: &FormatCommand) -> Self {
        if command.no_configuration {
            ConfigurationMode::Disabled
        } else {
            ConfigurationMode::Discover
        }
    }
}
