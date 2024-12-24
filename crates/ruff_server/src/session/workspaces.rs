// +------------------------------------------------------------+
// | Code adopted from:                                         |
// | Repository: https://github.com/astral-sh/ruff.git          |
// | Commit: 5bc9d6d3aa694ab13f38dd5cf91b713fd3844380           |
// +------------------------------------------------------------+

use std::path::Path;
use std::path::PathBuf;

use lsp_types::Url;
use lsp_types::WorkspaceFolder;
use workspace::resolve::PathResolver;
use workspace::resolve::SettingsResolver;
use workspace::settings::Settings;

/// Resolver for retrieving [`Settings`] associated with a workspace specific [`Path`]
#[derive(Debug, Default)]
pub(crate) struct WorkspaceSettingsResolver {
    /// Resolves a `path` to the closest workspace specific `SettingsResolver`.
    /// That `SettingsResolver` can then return `Settings` for the `path`.
    path_to_settings_resolver: PathResolver<SettingsResolver>,
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
        // If we fail for any reason (i.e. parse failure of an `air.toml`) then
        // we log an error and try to resolve the remaining workspace folders. We don't want
        // to propagate an error here because we don't want to prevent the server from
        // starting up entirely.
        // TODO: This is one place it would be nice to show a toast notification back
        // to the user, but we probably need to add support to the Aux thread for that?
        for workspace_folder in workspace_folders {
            if let Err(error) = resolver.open_workspace_folder(&workspace_folder.uri) {
                tracing::error!(
                    "Failed to load workspace settings for '{uri}':\n{error}",
                    uri = workspace_folder.uri.as_str(),
                    error = error
                );
            }
        }

        resolver
    }

    pub(crate) fn open_workspace_folder(&mut self, url: &Url) -> anyhow::Result<()> {
        let path = match Self::url_to_path(url)? {
            Some(path) => path,
            None => {
                tracing::warn!("Ignoring non-file workspace URL: {url}");
                return Ok(());
            }
        };

        // How to do better here?
        let fallback = Settings::default();

        let mut settings_resolver = SettingsResolver::new(fallback);
        settings_resolver.load_from_paths(&[&path])?;

        tracing::trace!("Adding workspace settings: {}", path.display());
        self.path_to_settings_resolver.add(&path, settings_resolver);

        Ok(())
    }

    pub(crate) fn close_workspace_folder(
        &mut self,
        url: &Url,
    ) -> anyhow::Result<Option<SettingsResolver>> {
        match Self::url_to_path(url)? {
            Some(path) => {
                tracing::trace!("Removing workspace settings: {}", path.display());
                Ok(self.path_to_settings_resolver.remove(&path))
            }
            None => {
                tracing::warn!("Ignoring non-file workspace URL: {url}");
                Ok(None)
            }
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.path_to_settings_resolver.len()
    }

    /// Return the appropriate [`Settings`] for a given document [`Url`].
    pub(crate) fn settings_for_url(&self, url: &Url) -> &Settings {
        if let Ok(Some(path)) = Self::url_to_path(url) {
            return self.settings_for_path(&path);
        }

        // For `untitled` schemes, we have special behavior.
        // If there is exactly 1 workspace, we resolve using a path of
        // `{workspace_path}/untitled` to provide relevant settings for this workspace.
        if url.scheme() == "untitled" && self.path_to_settings_resolver.len() == 1 {
            tracing::trace!("Using workspace settings for 'untitled' URL: {url}");
            let workspace_path = self.path_to_settings_resolver.keys().next().unwrap();
            let path = workspace_path.join("untitled");
            return self.settings_for_path(&path);
        }

        tracing::trace!("Using default settings for non-file URL: {url}");
        self.path_to_settings_resolver.fallback().fallback()
    }

    /// Reloads all workspaces matched by the [`Url`]
    ///
    /// This is utilized by the watched files handler to reload the settings
    /// resolver whenever an `air.toml` is modified.
    pub(crate) fn reload_workspaces_matched_by_url(&mut self, url: &Url) {
        let path = match Self::url_to_path(url) {
            Ok(Some(path)) => path,
            Ok(None) => {
                tracing::trace!("Ignoring non-`file` changed URL: {url}");
                return;
            }
            Err(error) => {
                tracing::error!("Failed to reload workspaces associated with {url}:\n{error}");
                return;
            }
        };

        if !path.ends_with("air.toml") {
            // We could get called with a changed file that isn't an `air.toml` if we are
            // watching more than `air.toml` files
            tracing::trace!("Ignoring non-`air.toml` changed URL: {url}");
            return;
        }

        for (workspace_path, settings_resolver) in self.path_to_settings_resolver.matches_mut(&path)
        {
            tracing::trace!("Reloading workspace settings: {}", workspace_path.display());

            settings_resolver.clear();

            if let Err(error) = settings_resolver.load_from_paths(&[workspace_path]) {
                tracing::error!(
                    "Failed to reload workspace settings for {path}:\n{error}",
                    path = workspace_path.display(),
                    error = error
                );
            }
        }
    }

    /// Return the appropriate [`Settings`] for a given [`Path`].
    ///
    /// This actually performs a double resolution. It first resolves to the
    /// workspace specific `SettingsResolver` that matches this path, and then uses that
    /// resolver to actually resolve the `Settings` for this path. We do it this way
    /// to ensure we can easily add and remove workspaces (including all of their
    /// hierarchical paths).
    fn settings_for_path(&self, path: &Path) -> &Settings {
        let settings_resolver = self.path_to_settings_resolver.resolve_or_fallback(path);
        settings_resolver.resolve_or_fallback(path)
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
