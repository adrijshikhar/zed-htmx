use std::fs;
use zed_extension_api::{self as zed, settings::LspSettings, LanguageServerId, Result};

const LANGUAGE_SERVER_ID: &str = "htmx-lsp";
const GITHUB_REPO: &str = "ThePrimeagen/htmx-lsp";

struct HtmxExtension {
    cached_binary_path: Option<String>,
}

impl HtmxExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        // 1. Check system PATH first
        if let Some(path) = worktree.which("htmx-lsp") {
            return Ok(path);
        }

        // 2. Check local in-memory cache
        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).is_ok_and(|stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        // 3. Query latest GitHub release
        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            GITHUB_REPO,
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (platform, arch) = zed::current_platform();
        let asset_name = match (platform, arch) {
            (zed::Os::Mac, zed::Architecture::Aarch64) => "htmx-lsp-darwin-arm64",
            (zed::Os::Mac, zed::Architecture::X8664) => "htmx-lsp-darwin-x64",
            (zed::Os::Linux, zed::Architecture::X8664) => "htmx-lsp-linux-x64",
            (zed::Os::Windows, zed::Architecture::Aarch64) => "htmx-lsp-windows-arm64.exe",
            (zed::Os::Windows, zed::Architecture::X8664) => "htmx-lsp-windows-x64.exe",
            _ => {
                return Err(format!(
                    "htmx-lsp does not publish prebuilt binaries for {platform:?}-{arch:?}. Please install htmx-lsp on your PATH (e.g., via `cargo install htmx-lsp`)."
                ));
            }
        };

        let asset = release
            .assets
            .iter()
            .find(|a| a.name == asset_name)
            .ok_or_else(|| {
                format!(
                    "no asset found matching {asset_name:?} in release {}",
                    release.version
                )
            })?;

        let version_dir = format!("{LANGUAGE_SERVER_ID}-{}", release.version);
        fs::create_dir_all(&version_dir)
            .map_err(|e| format!("failed to create directory {version_dir}: {e}"))?;

        let extension = match platform {
            zed::Os::Windows => ".exe",
            _ => "",
        };
        let binary_path = format!("{version_dir}/htmx-lsp{extension}");

        if !fs::metadata(&binary_path).is_ok_and(|stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &binary_path,
                zed::DownloadedFileType::Uncompressed,
            )
            .map_err(|e| format!("failed to download {asset_name}: {e}"))?;

            zed::make_file_executable(&binary_path)?;

            // Clean up older downloaded versions
            if let Ok(entries) = fs::read_dir(".") {
                for entry in entries.flatten() {
                    if let Some(file_name) = entry.file_name().to_str() {
                        if file_name.starts_with(LANGUAGE_SERVER_ID) && file_name != version_dir {
                            fs::remove_dir_all(entry.path()).ok();
                        }
                    }
                }
            }
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

impl zed::Extension for HtmxExtension {
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
        // Allow user configuration override from settings.json
        let binary_settings = LspSettings::for_worktree(LANGUAGE_SERVER_ID, worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.binary);

        let binary_args = binary_settings
            .as_ref()
            .and_then(|s| s.arguments.clone())
            .unwrap_or_default();

        let env = binary_settings
            .as_ref()
            .and_then(|s| s.env.clone())
            .map(|env| env.into_iter().collect())
            .unwrap_or_default();

        if let Some(path) = binary_settings.and_then(|s| s.path) {
            return Ok(zed::Command {
                command: path,
                args: binary_args,
                env,
            });
        }

        let binary_path = self.language_server_binary_path(language_server_id, worktree)?;

        Ok(zed::Command {
            command: binary_path,
            args: binary_args,
            env,
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        LspSettings::for_worktree(server_id.as_ref(), worktree)
            .map(|lsp_settings| lsp_settings.settings)
    }

    fn language_server_initialization_options(
        &mut self,
        server_id: &LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> Result<Option<zed_extension_api::serde_json::Value>> {
        LspSettings::for_worktree(server_id.as_ref(), worktree)
            .map(|lsp_settings| lsp_settings.initialization_options)
    }
}

zed::register_extension!(HtmxExtension);
