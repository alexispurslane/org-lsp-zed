use zed_extension_api as zed;

struct OrgLspExtension;

const LANGUAGE_SERVER_ID: &str = "org-lsp";
const BINARY_NAME: &str = "org-lsp";

impl zed::Extension for OrgLspExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let path = worktree
            .which(BINARY_NAME)
            .ok_or_else(|| {
                format!(
                    "org-lsp not found in PATH. Please install org-lsp from https://github.com/alexispurslane/org-lsp"
                )
            })?;

        Ok(zed::Command {
            command: path,
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
