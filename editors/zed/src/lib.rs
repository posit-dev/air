use std::fs;
use zed::LanguageServerId;
use zed_extension_api::{self as zed, settings::LspSettings, Result};

struct AirBinary {
    path: String,
    args: Option<Vec<String>>,
}

struct AirExtension {
    cached_binary_path: Option<String>,
}

#[derive(Debug, PartialEq)]
struct GithubReleaseDetails {
    /// The name of the GitHub asset that contains the binary
    asset_name: String,

    /// The type of file the asset is compressed as
    downloaded_file_type: zed::DownloadedFileType,

    /// The on disk directory the asset is extracted into, relative to the extension's
    /// working directory
    downloaded_directory: String,

    /// The on disk path to the binary. This is nested within `downloaded_directory`,
    /// and is relative to the extension's working directory.
    downloaded_binary_path: String,
}

impl AirExtension {
    fn language_server_binary(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<AirBinary> {
        // Pull `BinarySettings`, if they exist. This includes user specified path to the
        // binary and any user specified arguments for the binary.
        let binary_settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.binary);

        // We pass through user specified binary arguments no matter what method is
        // used to get the binary. If no arguments are supplied we eventually fall back to
        // just `language-server` as the sole argument.
        let binary_args = binary_settings
            .as_ref()
            .and_then(|binary_settings| binary_settings.arguments.clone());

        // Use user specified path to the `air` binary, if it is specified
        if let Some(path) = binary_settings.and_then(|binary_settings| binary_settings.path) {
            return Ok(AirBinary {
                path,
                args: binary_args,
            });
        }

        // Use binary on the `PATH`, if it exists
        if let Some(path) = worktree.which("air") {
            return Ok(AirBinary {
                path,
                args: binary_args,
            });
        }

        // Use binary from a previous download, if we can find one
        // (I'm not sure if this is cached across Zed sessions, or just within a single
        // Zed session when the server restarts)
        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).is_ok_and(|stat| stat.is_file()) {
                return Ok(AirBinary {
                    path: path.clone(),
                    args: binary_args,
                });
            }
        }

        // Ok, all other methods failed, we need to download the binary
        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let release = zed::latest_github_release(
            "posit-dev/air",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (platform, arch) = zed::current_platform();
        let release_details = GithubReleaseDetails::new(platform, arch, release.version);

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == release_details.asset_name)
            .ok_or_else(|| {
                format!(
                    "No asset found matching {asset_name:?}",
                    asset_name = release_details.asset_name
                )
            })?;

        if !fs::metadata(&release_details.downloaded_binary_path).is_ok_and(|stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &release_details.downloaded_directory,
                release_details.downloaded_file_type,
            )
            .map_err(|error| format!("Failed to download file: {error}"))?;

            // Clean out other entries in our personal extension directory, this may
            // include outdated versions of the extension, so it is good hygiene
            let entries = fs::read_dir(".")
                .map_err(|error| format!("Failed to list working directory: {error}"))?;

            for entry in entries {
                let entry =
                    entry.map_err(|error| format!("Failed to load directory entry: {error}"))?;
                if entry.file_name().to_str() != Some(&release_details.downloaded_directory) {
                    fs::remove_dir_all(entry.path()).ok();
                }
            }
        }

        // Update cache path for later
        self.cached_binary_path = Some(release_details.downloaded_binary_path.clone());

        Ok(AirBinary {
            path: release_details.downloaded_binary_path,
            args: binary_args,
        })
    }
}

impl GithubReleaseDetails {
    fn new(
        platform: zed_extension_api::Os,
        arch: zed_extension_api::Architecture,
        version: String,
    ) -> Self {
        let asset_stem = format!(
            "air-{arch}-{os}",
            arch = match arch {
                zed::Architecture::Aarch64 => "aarch64",
                zed::Architecture::X86 => "x86",
                zed::Architecture::X8664 => "x86_64",
            },
            os = match platform {
                zed::Os::Mac => "apple-darwin",
                zed::Os::Linux => "unknown-linux-gnu",
                zed::Os::Windows => "pc-windows-msvc",
            }
        );

        let asset_name = format!(
            "{asset_stem}.{suffix}",
            suffix = match platform {
                zed::Os::Mac | zed::Os::Linux => "tar.gz",
                zed::Os::Windows => "zip",
            }
        );

        let downloaded_file_type = match platform {
            zed::Os::Mac | zed::Os::Linux => zed::DownloadedFileType::GzipTar,
            zed::Os::Windows => zed::DownloadedFileType::Zip,
        };

        let downloaded_directory = format!("air-{version}");

        let downloaded_binary_path = match platform {
            zed::Os::Mac | zed::Os::Linux => format!("{downloaded_directory}/{asset_stem}/air"),
            zed::Os::Windows => format!("{downloaded_directory}/air.exe"),
        };

        Self {
            asset_name,
            downloaded_file_type,
            downloaded_directory,
            downloaded_binary_path,
        }
    }
}

impl zed::Extension for AirExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let air_binary = self.language_server_binary(language_server_id, worktree)?;
        Ok(zed::Command {
            command: air_binary.path,
            args: air_binary
                .args
                .unwrap_or_else(|| vec!["language-server".into()]),
            env: vec![],
        })
    }

    fn language_server_initialization_options(
        &mut self,
        server_id: &LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> Result<Option<zed_extension_api::serde_json::Value>> {
        let settings = LspSettings::for_worktree(server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.initialization_options.clone())
            .unwrap_or_default();
        Ok(Some(settings))
    }

    fn language_server_workspace_configuration(
        &mut self,
        server_id: &LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> Result<Option<zed_extension_api::serde_json::Value>> {
        let settings = LspSettings::for_worktree(server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();
        Ok(Some(settings))
    }
}

zed::register_extension!(AirExtension);

#[cfg(test)]
mod test {
    use crate::GithubReleaseDetails;

    /// Tests a few important variants across OSes and Arches
    #[test]
    fn test_github_release_details() {
        assert_eq!(
            GithubReleaseDetails::new(
                zed_extension_api::Os::Mac,
                zed_extension_api::Architecture::Aarch64,
                String::from("0.1.0"),
            ),
            GithubReleaseDetails {
                asset_name: String::from("air-aarch64-apple-darwin.tar.gz"),
                downloaded_file_type: zed_extension_api::DownloadedFileType::GzipTar,
                downloaded_directory: String::from("air-0.1.0"),
                downloaded_binary_path: String::from("air-0.1.0/air-aarch64-apple-darwin/air")
            }
        );

        assert_eq!(
            GithubReleaseDetails::new(
                zed_extension_api::Os::Linux,
                zed_extension_api::Architecture::X8664,
                String::from("0.2.0"),
            ),
            GithubReleaseDetails {
                asset_name: String::from("air-x86_64-unknown-linux-gnu.tar.gz"),
                downloaded_file_type: zed_extension_api::DownloadedFileType::GzipTar,
                downloaded_directory: String::from("air-0.2.0"),
                downloaded_binary_path: String::from("air-0.2.0/air-x86_64-unknown-linux-gnu/air")
            }
        );

        assert_eq!(
            GithubReleaseDetails::new(
                zed_extension_api::Os::Windows,
                zed_extension_api::Architecture::X86,
                String::from("0.1.0"),
            ),
            GithubReleaseDetails {
                asset_name: String::from("air-x86-pc-windows-msvc.zip"),
                downloaded_file_type: zed_extension_api::DownloadedFileType::Zip,
                downloaded_directory: String::from("air-0.1.0"),
                downloaded_binary_path: String::from("air-0.1.0/air.exe")
            }
        );
    }
}
