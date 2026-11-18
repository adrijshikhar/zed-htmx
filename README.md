# Zed HTMX Extension

[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

[HTMX](https://htmx.org/) language server extension for [Zed](https://zed.dev), powered by [`adrijshikhar/htmx-analyzer`](https://github.com/adrijshikhar/htmx-analyzer).

Provides intelligent completion, rich hover documentation, static analysis diagnostics, and quick-fixes for HTMX in HTML and server template languages.

---

## Installation

Install the extension directly from Zed:
1. Open the Command Palette (`Cmd+Shift+P` on macOS, `Ctrl+Shift+P` on Linux/Windows).
2. Select **`zed: extensions`**.
3. Search for **`HTMX`** and click **Install**.

On first use, the extension downloads the pinned, verified `htmx-analyzer` binary for your platform.

---

## Supported Languages

The extension registers `htmx-analyzer` across HTML and nine template languages. To use it with template languages, ensure the companion language extension is installed in Zed:

| Language | Zed Language Name | Companion Extension |
|---|---|---|
| HTML | `HTML` | Built-in |
| Jinja2 | `Jinja2` | `ArcherHume/jinja2-support` |
| Django | `Django` | `joshuadavidthomas/zed-django` |
| PHP | `PHP` | `zed-extensions/php` |
| ERB | `HTML+ERB` | `zed-extensions/ruby` |
| Blade | `Blade` | `bajrangCoder/zed-laravel-blade` |
| Vue | `Vue.js` | `zed-extensions/vue` |
| Astro | `Astro` | `zed-extensions/astro` |
| Svelte | `Svelte` | `zed-extensions/svelte` |
| Templ | `Templ` | `makifdb/zed-templ` |

---

## Configuration

The extension works out of the box with zero configuration required.

You can customize the binary location or server settings in your Zed `settings.json` under `lsp.htmx-analyzer`:

```json
{
  "lsp": {
    "htmx-analyzer": {
      "binary": {
        "path": "/usr/local/bin/htmx-analyzer"
      },
      "settings": {
        "diagnostics": {
          "severity": {
            "HTMX004": "warning"
          }
        }
      }
    }
  }
}
```

---

## Security

- **Pinned Releases and Hashes:** The server release tag and per-platform SHA-256 digests are compiled into `src/pinned.rs`. These hashes are generated only after verifying GitHub build provenance attestations (`gh attestation verify`) signed by the server repository's release workflow.
- **Download Verification:** Every downloaded binary is verified against its pinned SHA-256 digest before execution or permission changes. If the hash does not match, the downloaded file is deleted immediately and execution aborts with an error.
- **PATH Precedence:** If you have `htmx-analyzer` installed on your `PATH` (or specified via `lsp.htmx-analyzer.binary.path`), the extension uses it instead of downloading the pinned binary. Note that template language support (Astro, Svelte, Templ) requires `htmx-analyzer` ≥ 0.2.0.

---

## Development

### Prerequisites

- [Rust](https://rustup.rs/) (edition 2021)
- `wasm32-wasip2` target:
  ```bash
  rustup target add wasm32-wasip2
  ```

### Local Dev Installation in Zed

1. Open Zed.
2. Open the Command Palette (`Cmd+Shift+P` on macOS, `Ctrl+Shift+P` on Linux/Windows).
3. Run **`zed: install dev extension`**.
4. Select the `zed-htmx` repository directory.

---

## Credits

- Language server implementation: [adrijshikhar/htmx-analyzer](https://github.com/adrijshikhar/htmx-analyzer).
- Built for the [Zed](https://zed.dev) editor community.

---

## License

This project is licensed under the [Apache-2.0 License](LICENSE).
