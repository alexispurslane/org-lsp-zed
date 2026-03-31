use std::path::PathBuf;
use zed_extension_api as zed;

fn log(msg: &str) {
    eprintln!("[org-lsp-zed] {}", msg);
}

struct OrgLspExtension {
    cached_binary_path: Option<PathBuf>,
    cached_version: Option<String>,
}

const LANGUAGE_SERVER_ID: &str = "org-lsp";
const GITHUB_REPO: &str = "alexispurslane/org-lsp";

impl OrgLspExtension {
    fn get_platform_binary_name() -> String {
        let (os, arch) = zed::current_platform();

        let os_str = match os {
            zed::Os::Mac => "darwin",
            zed::Os::Linux => "linux",
            zed::Os::Windows => "windows",
        };

        let arch_str = match arch {
            zed::Architecture::Aarch64 => "arm64",
            zed::Architecture::X86 => "x86",
            zed::Architecture::X8664 => "x86_64",
        };

        let binary_name = format!("org-lsp-{}-{}", os_str, arch_str);
        log(&format!(
            "Detected platform: {}-{} -> {}",
            os_str, arch_str, binary_name
        ));

        // Windows binaries have .exe extension
        if os == zed::Os::Windows {
            format!("{}.exe", binary_name)
        } else {
            binary_name
        }
    }

    /// Fetches the latest release tag from GitHub API
    fn get_latest_version(&self) -> zed::Result<String> {
        let url = format!(
            "https://api.github.com/repos/{}/releases/latest",
            GITHUB_REPO
        );

        let request = zed::http_client::HttpRequestBuilder::new()
            .method(zed::http_client::HttpMethod::Get)
            .url(&url)
            .header("Accept", "application/vnd.github+json")
            .header("User-Agent", "org-lsp-zed")
            .build()
            .map_err(|e| format!("Failed to build HTTP request: {}", e))?;

        let response = zed::http_client::fetch(&request)
            .map_err(|e| format!("Failed to fetch release info: {}", e))?;

        let response_text = String::from_utf8_lossy(&response.body);

        // Parse the JSON to get the tag_name
        let json: zed::serde_json::Value = zed::serde_json::from_str(&response_text)
            .map_err(|e| format!("Failed to parse release JSON: {}", e))?;

        let tag_name = json["tag_name"]
            .as_str()
            .ok_or_else(|| "Failed to get tag_name from release".to_string())?
            .to_string();

        Ok(tag_name)
    }

    fn get_binary_path(&mut self, worktree: &zed::Worktree) -> zed::Result<PathBuf> {
        let binary_name = Self::get_platform_binary_name();
        let binary_path = PathBuf::from(&binary_name);

        log(&format!("Looking for binary: {:?}", binary_path));

        // First, check if the binary exists in the current directory (extension's working dir)
        if binary_path.exists() {
            log(&format!("Found existing binary at: {:?}", binary_path));
            self.cached_binary_path = Some(binary_path.clone());
            return Ok(binary_path);
        }

        log("Binary not found locally, checking for updates...");

        // Get the latest version from GitHub
        let latest_version = match self.get_latest_version() {
            Ok(v) => {
                log(&format!("Latest version: {}", v));
                v
            }
            Err(e) => {
                log(&format!("Failed to get latest version: {}", e));
                // If we can't fetch the latest version, try to use cached binary anyway
                if binary_path.exists() {
                    log("Using cached binary");
                    return Ok(binary_path);
                }
                return Err(format!(
                    "No cached binary available and cannot fetch: {}",
                    e
                ));
            }
        };

        // Check if we need to download/update the binary
        let version_changed = self.cached_version.as_ref() != Some(&latest_version);

        if binary_path.exists() && !version_changed {
            log("Binary exists and version matches - using cached");
            self.cached_binary_path = Some(binary_path.clone());
            return Ok(binary_path);
        }

        log("Downloading binary...");
        // Download the binary (new or update)
        self.download_binary(&binary_path, &latest_version, worktree)?;

        // Make the binary executable
        log("Making binary executable...");
        if let Err(e) = zed::make_file_executable(&binary_path.to_string_lossy()) {
            log(&format!("Warning: Failed to make binary executable: {}", e));
            // Continue anyway - the binary might already be executable
        }

        // Verify the binary exists after download
        if !binary_path.exists() {
            return Err(format!(
                "Binary not found after download: {:?}",
                binary_path
            ));
        }

        // Update the cached version
        self.cached_version = Some(latest_version);
        self.cached_binary_path = Some(binary_path.clone());
        log(&format!("Binary ready at: {:?}", binary_path));
        Ok(binary_path)
    }

    fn download_binary(
        &self,
        _binary_path: &PathBuf,
        version: &str,
        _worktree: &zed::Worktree,
    ) -> zed::Result<()> {
        let binary_name = Self::get_platform_binary_name();
        let download_url = format!(
            "https://github.com/{}/releases/download/{}/{}",
            GITHUB_REPO, version, binary_name
        );

        log(&format!("Downloading from: {}", download_url));

        // Download the binary (use relative path within extension's working directory)
        let relative_path = binary_name.as_str();
        match zed::download_file(
            &download_url,
            relative_path,
            zed::DownloadedFileType::Uncompressed,
        ) {
            Ok(_) => log("Download successful"),
            Err(e) => {
                log(&format!("Download failed: {}", e));
                return Err(format!("Failed to download binary: {}", e));
            }
        }

        Ok(())
    }
}

impl zed::Extension for OrgLspExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
            cached_version: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        log(&format!("Starting language server: {}", language_server_id));

        let path = self.get_binary_path(worktree)?;
        let cmd = path.to_string_lossy().to_string();

        log(&format!("Language server command: {} --stdio", cmd));

        Ok(zed::Command {
            command: cmd,
            args: vec!["--stdio".to_string()],
            env: worktree.shell_env(),
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        _server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<Option<zed::serde_json::Value>> {
        log("Getting workspace configuration");

        let settings = zed::settings::LspSettings::for_worktree(LANGUAGE_SERVER_ID, worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone());

        Ok(settings)
    }
}

zed::register_extension!(OrgLspExtension);
