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

impl AirExtension {
    fn language_server_binary(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<AirBinary> {
        // Pull `BinarySettings`, if they exist. This includes user specified path to the
        // binary and any user specified arguments for the binary.
        let binary_settings = LspSettings::for_worktree("air", worktree)
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
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
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
                zed::Os::Windows => "zip",
                _ => "tar.gz",
            }
        );

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        let version_dir = format!("air-{}", release.version);
        let binary_path = match platform {
            zed::Os::Windows => format!("{version_dir}/air.exe"),
            _ => format!("{version_dir}/{asset_stem}/air"),
        };

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            let file_kind = match platform {
                zed::Os::Windows => zed::DownloadedFileType::Zip,
                _ => zed::DownloadedFileType::GzipTar,
            };
            zed::download_file(&asset.download_url, &version_dir, file_kind)
                .map_err(|e| format!("failed to download file: {e}"))?;

            let entries =
                fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
                if entry.file_name().to_str() != Some(&version_dir) {
                    fs::remove_dir_all(entry.path()).ok();
                }
            }
        }

        // Update cache path for later
        self.cached_binary_path = Some(binary_path.clone());

        Ok(AirBinary {
            path: binary_path,
            args: binary_args,
        })
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
