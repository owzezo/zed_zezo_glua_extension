use std::fs;
use zed_extension_api::{self as zed, LanguageServerId, Result};

struct GluaExtension {
    cached_binary_path: Option<String>,
}

impl GluaExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        if let Some(path) = worktree.which("lua-language-server") {
            return Ok(path);
        }

        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            "LuaLS/lua-language-server",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (os, arch) = zed::current_platform();
        let version = release.version.clone();

        let (platform, ext) = match os {
            zed::Os::Mac => ("darwin", "tar.gz"),
            zed::Os::Linux => ("linux", "tar.gz"),
            zed::Os::Windows => ("win32", "zip"),
        };
        let arch = match arch {
            zed::Architecture::Aarch64 => "arm64",
            zed::Architecture::X8664 => "x64",
            zed::Architecture::X86 => "ia32",
        };

        let asset_name = format!("lua-language-server-{version}-{platform}-{arch}.{ext}");
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no LuaLS release asset matching '{asset_name}'"))?;

        let version_dir = format!("lua-language-server-{version}");
        let binary_path = format!("{version_dir}/bin/lua-language-server");

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            let file_type = match ext {
                "zip" => zed::DownloadedFileType::Zip,
                _ => zed::DownloadedFileType::GzipTar,
            };
            zed::download_file(&asset.download_url, &version_dir, file_type)?;
            zed::make_file_executable(&binary_path)?;
            remove_old_dirs("lua-language-server-", &version_dir);
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }

    // Grabs the LuaLS GMod definitions from glua-api-snippets and returns the
    // absolute directory to feed into Lua.workspace.library.
    fn gmod_definitions_path(&self, language_server_id: &LanguageServerId) -> Option<String> {
        let release = zed::latest_github_release(
            "luttje/glua-api-snippets",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )
        .ok()?;

        let asset_name = format!("{}.lua.zip", release.version);
        let asset = release.assets.iter().find(|a| a.name == asset_name)?;

        let dir = format!("gmod-defs-{}", release.version);
        let marker = format!("{dir}/.done");

        if !fs::metadata(&marker).map_or(false, |stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            zed::download_file(&asset.download_url, &dir, zed::DownloadedFileType::Zip).ok()?;
            fs::write(&marker, b"").ok();
            remove_old_dirs("gmod-defs-", &dir);
        }

        let cwd = std::env::current_dir().ok()?;
        Some(cwd.join(&dir).to_string_lossy().into_owned())
    }
}

fn remove_old_dirs(prefix: &str, keep: &str) {
    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if name.starts_with(prefix) && name.as_ref() != keep {
                fs::remove_dir_all(entry.path()).ok();
            }
        }
    }
}

impl zed::Extension for GluaExtension {
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
        let binary = self.language_server_binary_path(language_server_id, worktree)?;
        Ok(zed::Command {
            command: binary,
            args: vec![],
            env: Default::default(),
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let mut library = vec![];
        if let Some(defs) = self.gmod_definitions_path(language_server_id) {
            library.push(defs);
        }

        Ok(Some(serde_json::json!({
            "Lua": {
                "runtime": {
                    "version": "LuaJIT",
                    "nonstandardSymbol": ["!", "!=", "&&", "||", "//", "/**/", "continue"],
                    "special": { "include": "require" }
                },
                "workspace": {
                    "library": library,
                    "checkThirdParty": false
                },
                "diagnostics": {
                    "globals": [
                        "GM", "GAMEMODE", "SWEP", "ENT", "EFFECT", "TOOL", "SANDBOX",
                        "hook", "net", "util", "surface", "draw", "render", "vgui",
                        "concommand", "timer", "team", "player", "ents", "file"
                    ],
                    "disable": ["lowercase-global"]
                },
                "telemetry": { "enable": false }
            }
        })))
    }
}

zed::register_extension!(GluaExtension);
