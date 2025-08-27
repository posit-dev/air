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
}

/// Resolved [`WorkspaceSettings`] for a workspace specific [`Path`]
pub(crate) enum WorkspaceSettings<'resolver> {
    Toml(&'resolver Settings),
    Fallback(&'resolver Settings),
}

impl WorkspaceSettingsResolver {
    /// Construct a new workspace settings resolver from an initial set of workspace folders
    pub(crate) fn from_workspace_folders(workspace_folders: Vec<WorkspaceFolder>) -> Self {
        // How to do better here?
        let fallback = Settings::default();

        let settings_resolver_fallback = SettingsResolver::new(fallback);
        let path_to_settings_resolver = PathResolver::new(settings_resolver_fallback);

        let mut resolver = Self {
            path_to_settings_resolver,
        };

        // Add each workspace folder's settings into the resolver.
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

        // How to do better here?
        let fallback = Settings::default();

        let mut settings_resolver = SettingsResolver::new(fallback);

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
        WorkspaceSettings::Fallback(self.path_to_settings_resolver.fallback().fallback())
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
                || {
                    WorkspaceSettings::Fallback(
                        self.path_to_settings_resolver.fallback().fallback(),
                    )
                },
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
            WorkspaceSettings::Fallback(settings) => settings,
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
            WorkspaceSettings::Fallback(settings) => {
                // In the fallback case, merge with client provided `DocumentSettings`
                let format_options = settings.format.to_format_options(source);
                DocumentSettings::merge(format_options, document_settings)
            }
        }
    }
}
