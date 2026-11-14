# Zed HTMX Extension

[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

[HTMX](https://htmx.org/) language server extension for [Zed](https://zed.dev), powered by [`ThePrimeagen/htmx-lsp`](https://github.com/ThePrimeagen/htmx-lsp).

Provides intelligent completion and documentation for `hx-*` attributes in HTML templates.

---

## Features

- **Attribute Autocompletion:** Smart autocompletions for all core HTMX attributes (`hx-get`, `hx-post`, `hx-target`, `hx-swap`, `hx-trigger`, `hx-indicator`, and more).
- **Attribute Value Suggestions:** Contextual values for attributes such as `hx-swap` (`innerHTML`, `outerHTML`, `beforebegin`, `afterend`, etc.).
- **Hover Documentation:** In-editor documentation and usage guides when hovering over HTMX attributes.
- **Zero Configuration:** Automatically downloads the appropriate prebuilt `htmx-lsp` binary for your operating system and architecture on first use.
- **Seamless Coexistence:** Runs concurrently alongside Zed's built-in HTML language server, Emmet, and Tailwind CSS.
- **Offline / Custom Binary Fallback:** Detects `htmx-lsp` on your system `PATH` if already installed (e.g., via `cargo install htmx-lsp` or Homebrew).

---

## Supported Platforms

Prebuilt binaries are automatically fetched from GitHub releases for:
- **macOS:** Apple Silicon (`arm64`) and Intel (`x86_64`)
- **Linux:** `x86_64` (for Linux `arm64`, install `htmx-lsp` on your `PATH` via `cargo install htmx-lsp`)
- **Windows:** `x86_64` and `arm64`

---

## Configuration

The extension works out of the box with zero configuration required.

If you want to customize the language server (e.g., provide a custom binary path, custom arguments, or change log levels), you can configure it in your Zed `settings.json`:

```json
{
  "lsp": {
    "htmx-lsp": {
      "binary": {
        "path": "/usr/local/bin/htmx-lsp",
        "arguments": ["--level", "DEBUG"]
      }
    }
  }
}
```

---

## Development

### Prerequisites

- [Rust](https://rustup.rs/) (edition 2021)
- `wasm32-wasip2` target:
  ```bash
  rustup target add wasm32-wasip2
  ```

### Building the Extension

```bash
cargo build --target wasm32-wasip2 --release
```

### Local Dev Installation in Zed

1. Open Zed.
2. Open the Command Palette (`Cmd+Shift+P` on macOS, `Ctrl+Shift+P` on Linux/Windows).
3. Run `zed: install dev extension`.
4. Select the `zed-htmx` repository directory.

---

## Credits

- Language server implementation: [ThePrimeagen/htmx-lsp](https://github.com/ThePrimeagen/htmx-lsp).
- Created for the [Zed](https://zed.dev) editor community.

---

## License

This project is licensed under the [Apache-2.0 License](LICENSE).
