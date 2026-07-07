//
// workspaces.rs
//
// Copyright (C) 2025 Posit Software, PBC. All rights reserved.
//
//

use std::path::Path;
use std::path::PathBuf;

use air_r_formatter::context::RFormatOptions;
use tower_lsp::lsp_types::Url;
use tower_lsp::lsp_types::WorkspaceFolder;
use workspace::discovery::DiscoveredSettings;
use workspace::discovery::discover_settings;
use workspace::resolve::PathResolver;
use workspace::settings::Settings;
use workspace::toml::is_air_toml;

use crate::settings::DocumentSettings;

/// Convenience type for the inner resolver of path -> [`Settings`]
type SettingsResolver = PathResolver<Settings>;

/// Resolver for retrieving [`WorkspaceSettings`] associated with a workspace specific [`Path`]
#[derive(Debug, Default)]
pub(crate) struct WorkspaceSettingsResolver {
    /// Resolves a `path` to the closest workspace specific `SettingsResolver`.
    /// That `SettingsResolver` can then return `Settings` for the `path`.
    path_to_settings_resolver: PathResolver<SettingsResolver>,

    /// Settings for files that `path_to_settings_resolver` fails to resolve
    workspace_default_settings: WorkspaceDefaultSettings,
}

/// Resolved [`WorkspaceSettings`] for a workspace specific [`Path`]
#[derive(Debug)]
pub(crate) enum WorkspaceSettings<'resolver> {
    Toml(&'resolver Settings),
    Default(&'resolver Settings),
}

/// Owned variant of [WorkspaceSettings] used for persistent default settings
#[derive(Debug)]
enum WorkspaceDefaultSettings {
    /// Default settings created from a user level `air.toml`
    Toml(Settings),

    /// Default settings created from [Settings::default()]
    Default(Settings),
}

impl Default for WorkspaceDefaultSettings {
    fn default() -> Self {
        Self::Default(Settings::default())
    }
}

impl WorkspaceSettingsResolver {
    /// Construct a new workspace settings resolver from an initial set of workspace folders
    pub(crate) fn from_workspace_folders(workspace_folders: Vec<WorkspaceFolder>) -> Self {
        let path_to_settings_resolver = PathResolver::new();
        let workspace_default_settings = WorkspaceDefaultSettings::discover();

        let mut resolver = Self {
            path_to_settings_resolver,
            workspace_default_settings,
        };

        // Add each workspace folder's settings into the resolver
        for workspace_folder in workspace_folders {
            resolver.open_workspace_folder(&workspace_folder.uri)
        }

        resolver
    }

    /// Open a workspace folder
    ///
    /// If we fail for any reason (i.e. parse failure of an `air.toml`), we handle the
    /// failure internally. This allows us to:
    /// - Avoid preventing the server from starting up at all (which would happen if we
    ///   propagated an error up)
    /// - Control the toast notification sent to the user (TODO, see below)
    ///
    /// TODO: We should hook up `showMessage` so we can show the user a toast notification
    /// when something fails here, as failure means we can't load their TOML settings.
    pub(crate) fn open_workspace_folder(&mut self, url: &Url) {
        let failed_to_open_workspace_folder = |url, error| {
            tracing::error!("Failed to open workspace folder for '{url}':\n{error}");
        };

        let path = match Self::url_to_path(url) {
            Ok(Some(path)) => path,
            Ok(None) => {
                tracing::warn!("Ignoring non-file workspace URL '{url}'");
                return;
            }
            Err(error) => {
                failed_to_open_workspace_folder(url, error);
                return;
            }
        };

        let discovered_settings = match discover_settings(&[&path]) {
            Ok(discovered_settings) => discovered_settings,
            Err(error) => {
                failed_to_open_workspace_folder(url, error);
                return;
            }
        };

        let mut settings_resolver = SettingsResolver::new();

        for DiscoveredSettings {
            directory,
            settings,
        } in discovered_settings
        {
            settings_resolver.add(&directory, settings);
        }

        tracing::trace!("Adding workspace settings: {}", path.display());
        self.path_to_settings_resolver.add(&path, settings_resolver);
    }

    pub(crate) fn close_workspace_folder(&mut self, url: &Url) {
        match Self::url_to_path(url) {
            Ok(Some(path)) => {
                tracing::trace!("Removing workspace settings: {}", path.display());
                self.path_to_settings_resolver.remove(&path);
            }
            Ok(None) => {
                tracing::warn!("Ignoring non-file workspace URL: {url}");
            }
            Err(error) => {
                tracing::error!("Failed to close workspace folder for '{url}':\n{error}");
            }
        }
    }

    /// Return the appropriate [`WorkspaceSettings`] for a given document [`Url`].
    pub(crate) fn settings_for_url(&self, url: &Url) -> WorkspaceSettings<'_> {
        if let Ok(Some(path)) = Self::url_to_path(url) {
            return self.settings_for_path(&path);
        }

        // For `untitled` schemes, we have special behavior.
        // If there is exactly 1 workspace, we resolve using a path of
        // `{workspace_path}/untitled` to provide relevant settings for this workspace.
        if url.scheme() == "untitled" && self.path_to_settings_resolver.len() == 1 {
            tracing::trace!("Using workspace settings for 'untitled' URL: {url}");
            let workspace_path = self
                .path_to_settings_resolver
                .items()
                .first()
                .unwrap()
                .path();
            let path = workspace_path.join("untitled");
            return self.settings_for_path(&path);
        }

        tracing::trace!("Using default settings for non-file URL: {url}");
        self.workspace_default_settings.as_workspace_settings()
    }

    /// Reloads all workspaces matched by the [`Url`]
    ///
    /// This is utilized by the watched files handler to reload the settings
    /// resolver whenever an `air.toml` is modified.
    ///
    /// Returns whether an `air.toml` file was modified (currently doesn't check
    /// for content changes).
    pub(crate) fn reload_workspaces_matched_by_url(&mut self, url: &Url) -> bool {
        let path = match Self::url_to_path(url) {
            Ok(Some(path)) => path,
            Ok(None) => {
                tracing::trace!("Ignoring non-`file` changed URL: {url}");
                return false;
            }
            Err(error) => {
                tracing::error!("Failed to reload workspaces associated with '{url}':\n{error}");
                return false;
            }
        };

        if !is_air_toml(&path) {
            // We could get called with a changed file that isn't an `air.toml` if we are
            // watching more than `air.toml` files
            tracing::trace!("Ignoring non-`air.toml` changed URL: {url}");
            return false;
        }

        let mut changed = false;

        // The user level `air.toml` lives outside any workspace folder! We register
        // watchers for `{user_config_directory()}/air.toml`, so if we match against
        // that, rediscover the default settings.
        if path.parent() == workspace::config::user_config_directory().as_deref() {
            tracing::trace!("Reloading user level settings");
            self.workspace_default_settings = WorkspaceDefaultSettings::discover();
            changed = true;
        }

        for workspace_match in self.path_to_settings_resolver.matches_mut(&path) {
            // Clear existing settings up front, regardless of what happens when reloading.
            // Done in a tight scope to avoid simultaneous mutable and immutable borrows.
            {
                let workspace_settings_resolver = workspace_match.value_mut();
                workspace_settings_resolver.clear();
            }

            let workspace_path = workspace_match.path();

            tracing::trace!("Reloading workspace settings: {}", workspace_path.display());

            let discovered_settings = match discover_settings(&[workspace_path]) {
                Ok(discovered_settings) => discovered_settings,
                Err(error) => {
                    let workspace_path = workspace_path.display();
                    tracing::error!("Failed to reload workspace for '{workspace_path}':\n{error}");
                    continue;
                }
            };

            // Now add in all rediscovered settings
            let workspace_settings_resolver = workspace_match.value_mut();

            for DiscoveredSettings {
                directory,
                settings,
            } in discovered_settings
            {
                changed = true;
                workspace_settings_resolver.add(&directory, settings);
            }
        }

        changed
    }

    /// Return the appropriate [`WorkspaceSettings`] for a given [`Path`].
    ///
    /// This actually performs a double resolution. It first resolves to the
    /// workspace specific `SettingsResolver` that matches this path, and then uses that
    /// resolver to actually resolve the `Settings` for this path. We do it this way
    /// to ensure we can easily add and remove workspaces (including all of their
    /// hierarchical paths).
    fn settings_for_path(&self, path: &Path) -> WorkspaceSettings<'_> {
        self.path_to_settings_resolver
            .resolve(path)
            .and_then(|resolution| resolution.value().resolve(path))
            .map_or_else(
                || self.workspace_default_settings.as_workspace_settings(),
                |resolution| WorkspaceSettings::Toml(resolution.value()),
            )
    }

    fn url_to_path(url: &Url) -> anyhow::Result<Option<PathBuf>> {
        if url.scheme() != "file" {
            return Ok(None);
        }

        let path = url
            .to_file_path()
            .map_err(|()| anyhow::anyhow!("Failed to convert workspace URL to file path: {url}"))?;

        Ok(Some(path))
    }
}

impl WorkspaceSettings<'_> {
    pub(crate) fn settings(&self) -> &Settings {
        match self {
            WorkspaceSettings::Toml(settings) => settings,
            WorkspaceSettings::Default(settings) => settings,
        }
    }

    pub(crate) fn to_format_options(
        &self,
        source: &str,
        document_settings: &DocumentSettings,
    ) -> RFormatOptions {
        match self {
            WorkspaceSettings::Toml(settings) => {
                // If there is an actual TOML, that wins
                settings.format.to_format_options(source)
            }
            WorkspaceSettings::Default(settings) => {
                // In the default case, merge with client provided `DocumentSettings`
                let format_options = settings.format.to_format_options(source);
                DocumentSettings::merge(format_options, document_settings)
            }
        }
    }
}

impl WorkspaceDefaultSettings {
    fn discover() -> Self {
        match workspace::discovery::discover_user_settings() {
            Ok(Some(settings)) => Self::Toml(settings),
            Ok(None) => Self::Default(Settings::default()),
            Err(error) => {
                tracing::error!("Failed to load user level air.toml:\n{error}");
                Self::Default(Settings::default())
            }
        }
    }

    fn as_workspace_settings(&self) -> WorkspaceSettings<'_> {
        match self {
            WorkspaceDefaultSettings::Toml(settings) => WorkspaceSettings::Toml(settings),
            WorkspaceDefaultSettings::Default(settings) => WorkspaceSettings::Default(settings),
        }
    }
}

#[cfg(test)]
mod test {
    use assert_matches::assert_matches;
    use tempfile::TempDir;
    use tower_lsp::lsp_types::Url;
    use tower_lsp::lsp_types::WorkspaceFolder;
    use workspace::config::set_user_config_directory_env_var;

    use crate::workspaces::WorkspaceSettings;
    use crate::workspaces::WorkspaceSettingsResolver;

    #[test]
    fn test_user_settings_used_as_fallback() {
        let config_directory = TempDir::new().unwrap();
        let config_directory = config_directory.path();
        unsafe { set_user_config_directory_env_var(config_directory) };

        let air_config_directory = config_directory.join("air");
        std::fs::create_dir_all(&air_config_directory).unwrap();

        std::fs::write(
            air_config_directory.join("air.toml"),
            "[format]
        indent-width = 3",
        )
        .unwrap();

        // No workspace folders, but we do have a user level `air.toml`, so we get that
        let resolver = WorkspaceSettingsResolver::from_workspace_folders(vec![]);

        let workspace = TempDir::new().unwrap();
        let path = workspace.path().join("foo.R");

        assert_matches!(resolver.settings_for_path(&path), WorkspaceSettings::Toml(settings) => {
            assert_eq!(settings.format.indent_width.value(), 3);
        });
    }

    #[test]
    fn test_default_used_when_no_user_settings() {
        // No `air.toml` written, but folder exists
        let config_directory = TempDir::new().unwrap();
        let config_directory = config_directory.path();
        unsafe { set_user_config_directory_env_var(config_directory) };

        let air_config_directory = config_directory.join("air");
        std::fs::create_dir_all(&air_config_directory).unwrap();

        let resolver = WorkspaceSettingsResolver::from_workspace_folders(vec![]);

        let workspace = TempDir::new().unwrap();
        let path = workspace.path().join("foo.R");

        assert_matches!(
            resolver.settings_for_path(&path),
            WorkspaceSettings::Default(_)
        );
    }

    #[test]
    fn test_project_settings_win_over_user_settings() {
        let config_directory = TempDir::new().unwrap();
        let config_directory = config_directory.path();
        unsafe { set_user_config_directory_env_var(config_directory) };

        let air_config_directory = config_directory.join("air");
        std::fs::create_dir_all(&air_config_directory).unwrap();

        std::fs::write(
            air_config_directory.join("air.toml"),
            "[format]
        indent-width = 3",
        )
        .unwrap();

        let workspace = TempDir::new().unwrap();
        let workspace = workspace.path();
        std::fs::write(
            workspace.join("air.toml"),
            "[format]
        indent-width = 5",
        )
        .unwrap();

        let folder = WorkspaceFolder {
            uri: Url::from_directory_path(workspace).unwrap(),
            name: String::from("workspace"),
        };
        let resolver = WorkspaceSettingsResolver::from_workspace_folders(vec![folder]);

        let path = workspace.join("foo.R");

        // The project `air.toml` shadows the user level one
        assert_matches!(resolver.settings_for_path(&path), WorkspaceSettings::Toml(settings) => {
            assert_eq!(settings.format.indent_width.value(), 5);
        });
    }

    #[test]
    fn test_reload_of_user_settings_handles_create_edit_delete() {
        let config_directory = TempDir::new().unwrap();
        let config_directory = config_directory.path();
        unsafe { set_user_config_directory_env_var(config_directory) };

        let air_config_directory = config_directory.join("air");
        std::fs::create_dir_all(&air_config_directory).unwrap();

        let air_toml_path = air_config_directory.join("air.toml");
        let air_toml_url = Url::from_file_path(&air_toml_path).unwrap();

        // Initially no user level `air.toml`, so we fall back to defaults
        let mut resolver = WorkspaceSettingsResolver::from_workspace_folders(vec![]);

        let workspace = TempDir::new().unwrap();
        let path = workspace.path().join("foo.R");
        assert_matches!(
            resolver.settings_for_path(&path),
            WorkspaceSettings::Default(_)
        );

        // Create: pick up user settings
        std::fs::write(
            &air_toml_path,
            "[format]
        indent-width = 3",
        )
        .unwrap();
        assert!(resolver.reload_workspaces_matched_by_url(&air_toml_url));
        assert_matches!(resolver.settings_for_path(&path), WorkspaceSettings::Toml(settings) => {
            assert_eq!(settings.format.indent_width.value(), 3);
        });

        // Edit: picks up the new settings
        std::fs::write(
            &air_toml_path,
            "[format]
        indent-width = 4",
        )
        .unwrap();
        assert!(resolver.reload_workspaces_matched_by_url(&air_toml_url));
        assert_matches!(resolver.settings_for_path(&path), WorkspaceSettings::Toml(settings) => {
            assert_eq!(settings.format.indent_width.value(), 4);
        });

        // Delete: downgrades to default
        std::fs::remove_file(&air_toml_path).unwrap();
        assert!(resolver.reload_workspaces_matched_by_url(&air_toml_url));
        assert_matches!(
            resolver.settings_for_path(&path),
            WorkspaceSettings::Default(_)
        );
    }
}
