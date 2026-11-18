#![forbid(unsafe_code)]

mod install;
mod pinned;

use std::fs;
use zed_extension_api::{self as zed, settings::LspSettings, LanguageServerId, Result};

const LANGUAGE_SERVER_ID: &str = "htmx-analyzer";

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
        if let Some(path) = worktree.which("htmx-analyzer") {
            return Ok(path);
        }

        // 2. Check local in-memory cache
        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).is_ok_and(|stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        // 3. Platform check
        let (os, arch) = zed::current_platform();
        let key = install::platform_key(os, arch)
            .ok_or_else(|| install::unsupported_platform_message(os, arch))?;
        let (asset_name, expected_sha) = install::pinned_asset(key)
            .ok_or_else(|| install::unsupported_platform_message(os, arch))?;

        let version_dir = format!("htmx-analyzer-{}", pinned::SERVER_TAG);
        let extension = match os {
            zed::Os::Windows => ".exe",
            _ => "",
        };
        let binary_path = format!("{version_dir}/htmx-analyzer{extension}");

        if !fs::metadata(&binary_path).is_ok_and(|stat| stat.is_file()) {
            fs::create_dir_all(&version_dir)
                .map_err(|e| format!("failed to create directory {version_dir}: {e}"))?;

            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            let download_url = format!(
                "https://github.com/adrijshikhar/htmx-analyzer/releases/download/{}/{asset_name}",
                pinned::SERVER_TAG
            );
            let temp_binary_path = format!("{binary_path}.download");

            let download_result = zed::download_file(
                &download_url,
                &temp_binary_path,
                zed::DownloadedFileType::Uncompressed,
            );

            if let Err(e) = download_result {
                let _ = fs::remove_file(&temp_binary_path);
                return Err(format!("failed to download {asset_name}: {e}"));
            }

            if let Err(e) = install::verify_download(&temp_binary_path, expected_sha) {
                let _ = fs::remove_file(&temp_binary_path);
                return Err(e);
            }

            if let Err(e) = zed::make_file_executable(&temp_binary_path) {
                let _ = fs::remove_file(&temp_binary_path);
                return Err(e);
            }

            if let Err(e) = fs::rename(&temp_binary_path, &binary_path) {
                let _ = fs::remove_file(&temp_binary_path);
                return Err(format!("failed to finalize binary installation: {e}"));
            }

            // Delete stale dirs only after successful install
            if let Ok(entries) = fs::read_dir(".") {
                for entry in entries.flatten() {
                    if let Ok(file_name) = entry.file_name().into_string() {
                        if entry.path().is_dir()
                            && install::is_stale_install_dir(&file_name, &version_dir)
                        {
                            let _ = fs::remove_dir_all(entry.path());
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

        let (binary_path_override, binary_args, env) = match binary_settings {
            Some(settings) => {
                let args = settings
                    .arguments
                    .unwrap_or_else(|| vec!["--stdio".to_string()]);
                let env = settings
                    .env
                    .map(|m| m.into_iter().collect())
                    .unwrap_or_default();
                (settings.path, args, env)
            }
            None => (None, vec!["--stdio".to_string()], Vec::new()),
        };

        let command = match binary_path_override {
            Some(path) => path,
            None => self.language_server_binary_path(language_server_id, worktree)?,
        };

        Ok(zed::Command {
            command,
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
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        LspSettings::for_worktree(server_id.as_ref(), worktree)
            .map(|lsp_settings| lsp_settings.initialization_options)
    }
}

zed::register_extension!(HtmxExtension);
