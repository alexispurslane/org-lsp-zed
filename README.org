* org-lsp for Zed
:PROPERTIES:
:CUSTOM_ID: org-lsp-for-zed
:ID: f8146f41-b33c-422e-a998-f0671d2fd235
:END:


A Zed extension that provides Language Server Protocol (LSP) support for
Org mode files.

** Overview
:PROPERTIES:
:CUSTOM_ID: overview
:ID: a4e5a37f-4399-48c4-b3e6-90769142a40b
:END:


This extension provides an LSP client for the [[https://github.com/alexispurslane/org-lsp][org-lsp]] language server,
adding advanced IDE features to Org mode files in Zed:
- *Navigation*: Go-to-definition for =file:= and =id:= links, document
  symbols, workspace symbols
- *Completion*: Auto-completion for tags, file links, ID links, block
  types, export formats
- *Hover*: Preview link destinations without leaving your current file
- *Formatting*: Document formatting with proper spacing and alignment
- *Folding*: Code folding for headings and sections
- *Code Actions*: convert text into a link, add deadlines, clock in or out, add properties, add custom ids, transform heading trees into nested lists and vice versa, and more!
- *Syntax Highlighting*: a vendored and updated version of https://github.com/emiasims/tree-sitter-org


** Prerequisites
:PROPERTIES:
:CUSTOM_ID: prerequisites
:ID: de65f775-2e7b-4200-9206-507ba3b120b6
:END:


*** 1. Install the Org Language Extension
:PROPERTIES:
:CUSTOM_ID: install-the-org-language-extension
:ID: 8a91dbdc-0c92-487d-9fb7-883e383610b1
:END:


This extension provides LSP support only. You must also install the [[https://github.com/hron/zed-org][org]]
extension for syntax highlighting and Tree-sitter support:
#+BEGIN_EXAMPLE
zed: install extension org
#+END_EXAMPLE

Or search for "org" in the Zed extension gallery (by hron).

*** 2. Install the org-lsp Binary
:PROPERTIES:
:CUSTOM_ID: install-the-org-lsp-binary
:ID: 75161fa8-c4f9-4a75-894f-8c5f4c552054
:END:


This extension requires the [[https://github.com/alexispurslane/org-lsp][org-lsp]] binary to be installed and
available in your PATH.

*From source:*
#+BEGIN_EXAMPLE
git clone https://github.com/alexispurslane/org-lsp.git
cd org-lsp
just install  # Installs to ~/.local/bin/org-lsp
#+END_EXAMPLE

Ensure=~/.local/bin= is in your PATH.

*Requirements:*
- Go 1.25.6 or later
- =just= task runner (=cargo install just=)


** Installation
:PROPERTIES:
:CUSTOM_ID: installation
:ID: b3f0a6db-9688-42d2-bb18-2775dc26827c
:END:


This extension is not yet published in the Zed extension gallery. Install
it as a dev extension:
1. Clone this repository
2. Build the extension: =cargo build --target wasm32-wasi=
3. In Zed, run =zed: install dev extension=
4. Select the =org-lsp-zed= directory


** Configuration
:PROPERTIES:
:CUSTOM_ID: configuration
:ID: 1711d76c-ccf4-4e2c-9e4c-914bf8f95ae5
:END:


You can configure the LSP settings in your Zed=settings.json=:
#+BEGIN_SRC json
{
  "lsp": {
    "org-lsp": {
      "settings": {}
    }
  }
}
#+END_SRC

** How It Works
:PROPERTIES:
:CUSTOM_ID: how-it-works
:ID: 68350c49-a181-4d5e-bca1-dbb47152d210
:END:


This extension is designed to complement the existing [[https://github.com/hron/zed-org][org]] extension
(which provides syntax highlighting and grammar support). It:
1. Detects when you open an =.org= file
2. Checks if =org-lsp= is available in your PATH
3. Starts the language server with stdio communication
4. Provides all LSP features (go-to-def, completion, hover, etc.)


** Troubleshooting
:PROPERTIES:
:CUSTOM_ID: troubleshooting
:ID: a3dd27c8-b427-4531-9184-c646dd313181
:END:


*"org-lsp not found in PATH"*

Make sure [[https://github.com/alexispurslane/org-lsp][org-lsp]] is installed and in your PATH. You can verify by
running:
#+BEGIN_EXAMPLE
which org-lsp
org-lsp --version
#+END_EXAMPLE

*No LSP features working*
- Check Zed LSP logs: =zed: open log=
- Ensure you have both the [[https://github.com/hron/zed-org][org]] extension (for language support) and
  =org-lsp= extension installed


** Development
:PROPERTIES:
:CUSTOM_ID: development
:ID: 86b294f0-3605-42a0-818a-e17332e80835
:END:

#+BEGIN_EXAMPLE
# Clone the repository
git clone https://github.com/alexispurslane/org-lsp-zed.git
cd org-lsp-zed

# Build the extension
cargo build --target wasm32-wasi

# Install as dev extension in Zed
# Run: zed: install dev extension and select this directory
#+END_EXAMPLE

** License
:PROPERTIES:
:CUSTOM_ID: license
:ID: f471bd6f-7245-4336-a78c-adf9d4ba285e
:END:


This is free and unencumbered software released into the public domain.
See [[./UNLICENSE][UNLICENSE]] for details.
