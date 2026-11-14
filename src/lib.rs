#![forbid(unsafe_code)]

use std::fs;
use zed_extension_api::{self as zed, settings::LspSettings, LanguageServerId, Result};

const LANGUAGE_SERVER_ID: &str = "htmx-lsp";
const GITHUB_REPO: &str = "adrijshikhar/htmx-analyzer";

struct HtmxExtension {
    cached_binary_path: Option<String>,
}

impl HtmxExtension {
    fn find_existing_binary(&self) -> Option<String> {
        let (platform, _) = zed::current_platform();
        let extension = match platform {
            zed::Os::Windows => ".exe",
            _ => "",
        };
        let target_filenames = [
            format!("htmx-analyzer{extension}"),
            format!("htmx-lsp{extension}"),
        ];

        if let Ok(entries) = fs::read_dir(".") {
            let mut dirs: Vec<_> = entries
                .flatten()
                .filter(|entry| {
                    if let Some(name) = entry.file_name().to_str() {
                        (name.starts_with(LANGUAGE_SERVER_ID) || name.starts_with("htmx-analyzer"))
                            && entry.path().is_dir()
                    } else {
                        false
                    }
                })
                .collect();

            // Sort descending by directory name so latest release version is prioritized
            dirs.sort_by_key(|b| std::cmp::Reverse(b.file_name()));

            for entry in dirs {
                for target in &target_filenames {
                    let binary_path = entry.path().join(target);
                    if binary_path.is_file() {
                        return Some(binary_path.to_string_lossy().to_string());
                    }
                }
            }
        }
        None
    }

    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        // 1. Check system PATH first
        if let Some(path) = worktree.which("htmx-analyzer") {
            return Ok(path);
        }
        if let Some(path) = worktree.which("htmx-lsp") {
            return Ok(path);
        }

        // 2. Check local in-memory cache
        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).is_ok_and(|stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        // 3. Validate platform support before attempting any network query
        let (platform, arch) = zed::current_platform();
        let asset_suffix = match (platform, arch) {
            (zed::Os::Mac, zed::Architecture::Aarch64) => "darwin-arm64",
            (zed::Os::Mac, zed::Architecture::X8664) => "darwin-x64",
            (zed::Os::Linux, zed::Architecture::X8664) => "linux-x64",
            (zed::Os::Windows, zed::Architecture::X8664) => "windows-x64.exe",
            _ => {
                if let Some(existing) = self.find_existing_binary() {
                    self.cached_binary_path = Some(existing.clone());
                    return Ok(existing);
                }
                return Err(format!(
                    "htmx-analyzer does not publish prebuilt binaries for {platform:?}-{arch:?}. Please install htmx-analyzer on your PATH."
                ));
            }
        };
        let asset_name = format!("htmx-analyzer-{asset_suffix}");

        // 4. Query latest GitHub release (with offline fallback to existing on-disk binary)
        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = match zed::latest_github_release(
            GITHUB_REPO,
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        ) {
            Ok(r) => r,
            Err(e) => {
                if let Some(existing) = self.find_existing_binary() {
                    self.cached_binary_path = Some(existing.clone());
                    return Ok(existing);
                }
                return Err(e);
            }
        };

        let asset = match release.assets.iter().find(|a| a.name == asset_name) {
            Some(a) => a,
            None => {
                if let Some(existing) = self.find_existing_binary() {
                    self.cached_binary_path = Some(existing.clone());
                    return Ok(existing);
                }
                return Err(format!(
                    "no asset found matching {asset_name:?} in release {}",
                    release.version
                ));
            }
        };

        let version_dir = format!("htmx-analyzer-{}", release.version);
        fs::create_dir_all(&version_dir)
            .map_err(|e| format!("failed to create directory {version_dir}: {e}"))?;

        let extension = match platform {
            zed::Os::Windows => ".exe",
            _ => "",
        };
        let binary_path = format!("{version_dir}/htmx-analyzer{extension}");

        if !fs::metadata(&binary_path).is_ok_and(|stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            // Atomic download: download to .download staging file, chmod, then rename
            let temp_binary_path = format!("{binary_path}.download");
            let download_result = zed::download_file(
                &asset.download_url,
                &temp_binary_path,
                zed::DownloadedFileType::Uncompressed,
            );

            if let Err(e) = download_result {
                fs::remove_file(&temp_binary_path).ok();
                if let Some(existing) = self.find_existing_binary() {
                    self.cached_binary_path = Some(existing.clone());
                    return Ok(existing);
                }
                return Err(format!("failed to download {asset_name}: {e}"));
            }

            if let Err(e) = zed::make_file_executable(&temp_binary_path) {
                fs::remove_file(&temp_binary_path).ok();
                if let Some(existing) = self.find_existing_binary() {
                    self.cached_binary_path = Some(existing.clone());
                    return Ok(existing);
                }
                return Err(e);
            }

            if let Err(e) = fs::rename(&temp_binary_path, &binary_path) {
                fs::remove_file(&temp_binary_path).ok();
                if let Some(existing) = self.find_existing_binary() {
                    self.cached_binary_path = Some(existing.clone());
                    return Ok(existing);
                }
                return Err(format!("failed to finalize binary installation: {e}"));
            }

            // Clean up older downloaded versions
            if let Ok(entries) = fs::read_dir(".") {
                for entry in entries.flatten() {
                    if let Some(file_name) = entry.file_name().to_str() {
                        if (file_name.starts_with("htmx-analyzer-")
                            || file_name.starts_with(LANGUAGE_SERVER_ID))
                            && file_name != version_dir
                            && entry.path().is_dir()
                        {
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
            .unwrap_or_else(|| vec!["--stdio".to_string()]);

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
