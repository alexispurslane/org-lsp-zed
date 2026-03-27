use std::path::PathBuf;
use zed_extension_api as zed;

struct OrgLspExtension {
    cached_binary_path: Option<PathBuf>,
}

const LANGUAGE_SERVER_ID: &str = "org-lsp";
const GITHUB_REPO: &str = "alexispurslane/org-lsp";

impl OrgLspExtension {
    fn get_platform_binary_name() -> &'static str {
        match (std::env::consts::OS, std::env::consts::ARCH) {
            ("macos", "aarch64") => "org-lsp-darwin-arm64",
            ("macos", "x86_64") => "org-lsp-darwin-x86_64",
            ("linux", "aarch64") => "org-lsp-linux-arm64",
            ("linux", "x86_64") => "org-lsp-linux-x86_64",
            ("windows", "x86_64") => "org-lsp-windows-x86_64.exe",
            _ => "org-lsp-linux-x86_64",
        }
    }

    fn get_binary_path(&mut self, worktree: &zed::Worktree) -> zed::Result<PathBuf> {
        // If we already have the binary cached, return it
        if let Some(ref path) = self.cached_binary_path {
            if path.exists() {
                return Ok(path.clone());
            }
        }

        // Use the binary name as the relative path within the extension's working directory
        let binary_name = Self::get_platform_binary_name();
        let binary_path = PathBuf::from(binary_name);

        // Check if binary already exists (in extension's working directory)
        if binary_path.exists() {
            self.cached_binary_path = Some(binary_path.clone());
            return Ok(binary_path);
        }

        // Download the binary
        self.download_binary(&binary_path, worktree)?;

        // Make the binary executable
        zed::make_file_executable(&binary_path.to_string_lossy())
            .map_err(|e| format!("Failed to make binary executable: {}", e))?;

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }

    fn download_binary(
        &self,
        _binary_path: &PathBuf,
        _worktree: &zed::Worktree,
    ) -> zed::Result<()> {
        // Fetch the latest release from GitHub API
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
            .ok_or_else(|| "Failed to get tag_name from release".to_string())?;

        let binary_name = Self::get_platform_binary_name();
        let download_url = format!(
            "https://github.com/{}/releases/download/{}/{}",
            GITHUB_REPO, tag_name, binary_name
        );

        // Download the binary (use relative path within extension's working directory)
        let relative_path = binary_name;
        zed::download_file(
            &download_url,
            relative_path,
            zed::DownloadedFileType::Uncompressed,
        )
        .map_err(|e| format!("Failed to download binary: {}", e))?;

        Ok(())
    }
}

impl zed::Extension for OrgLspExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let path = self.get_binary_path(worktree)?;

        Ok(zed::Command {
            command: path.to_string_lossy().to_string(),
            args: vec!["--stdio".to_string()],
            env: worktree.shell_env(),
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        _server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<Option<zed::serde_json::Value>> {
        let settings = zed::settings::LspSettings::for_worktree(LANGUAGE_SERVER_ID, worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone());

        Ok(settings)
    }
}

zed::register_extension!(OrgLspExtension);
