# org-lsp for Zed

A Zed extension that provides Language Server Protocol (LSP) support for Org mode files.

## Overview

This extension provides an LSP client for the `org-lsp` language server, adding advanced IDE features to Org mode files in Zed:

- **Navigation**: Go-to-definition for `file:` and `id:` links, document symbols, workspace symbols
- **Completion**: Auto-completion for tags, file links, ID links, block types, export formats
- **Hover**: Preview link destinations without leaving your current file
- **Formatting**: Document formatting with proper spacing and alignment
- **Folding**: Code folding for headings and sections

## Prerequisites

### 1. Install the Org Language Extension

This extension provides LSP support only. You must also install the Org language extension for syntax highlighting and Tree-sitter support:

    zedd: install extension org

Or search for "org" in the Zed extension gallery (by hron).

### 2. Install the org-lsp Binary

This extension requires the `org-lsp` binary to be installed and available in your PATH.

**From source:**

    git clone https://github.com/alexispurslane/org-lsp.git
    cd org-lsp
    just install  # Installs to ~/.local/bin/org-lsp

Ensure ~/.local/bin is in your PATH.

**Requirements:**
- Go 1.25.6 or later
- `just` task runner (`cargo install just`)

## Installation

Install this extension from the Zed extension gallery:

    zedd: install extension org-lsp

Or install as a dev extension during development:

1. Clone this repository
2. In Zed, run `zed: install dev extension`
3. Select the `org-lsp-zed` directory

## Configuration

You can configure the LSP settings in your Zed settings.json:

```json
{
  "lsp": {
    "org-lsp": {
      "settings": {}
    }
  }
}
```

## How It Works

This extension is designed to complement the existing `org` extension (which provides syntax highlighting and grammar support). It:

1. Detects when you open an `.org` file
2. Checks if `org-lsp` is available in your PATH
3. Starts the language server with stdio communication
4. Provides all LSP features (go-to-def, completion, hover, etc.)

## Troubleshooting

**"org-lsp not found in PATH"**

Make sure `org-lsp` is installed and in your PATH. You can verify by running:

    which org-lsp
    org-lsp --version

**No LSP features working**

- Check Zed LSP logs: `zed: open log`
- Ensure you have both the `org` extension (for language support) and `org-lsp` extension installed

## Development

    # Clone the repository
    git clone https://github.com/alexispurslane/org-lsp-zed.git
    cd org-lsp-zed

    # Build the extension
    cargo build --target wasm32-wasi

    # Install as dev extension in Zed
    # Run: zed: install dev extension and select this directory

## License

This is free and unencumbered software released into the public domain. See UNLICENSE for details.

## Related Projects

- [org-lsp](https://github.com/alexispurslane/org-lsp) - The language server this extension wraps
- [hron/zed-org](https://github.com/hron/zed-org) - The Org mode language extension for Zed
- [Org mode](https://orgmode.org/) - The original Emacs Org mode
