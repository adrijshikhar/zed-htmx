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

## Performance

Engineered in pure Rust for Zed's signature zero-latency editing experience. `htmx-analyzer` pairs a zero-allocation linear attribute lexer, incremental rope VFS, and non-blocking JSON-RPC transport to deliver instantaneous feedback with an almost invisible system footprint:
- **Cold Startup:** `2.6 ms` (p50)
- **Base Memory:** `2.9 MB` idle RSS
- **Binary Footprint:** `~2.4 MB` standalone native binary with zero runtime dependencies

### Executive Performance Scoreboard

| Benchmark Dimension | Measurement | Zed Editing Experience |
| :--- | :--- | :--- |
| **Cold Startup** | **`2.6 ms`** (p50) | Near-instant language server readiness upon file open |
| **Sub-Millisecond Queries** | **Hover:** `0.098 – 0.481 ms`<br>**Completion:** `0.100 – 0.489 ms`<br>**Cross-File Target ID:** `0.040 ms` | Keystroke completions and hover tooltips render well below 60fps frame times (16.6 ms) |
| **Massive Scale Parsing** | **1,000 to 100,000 lines**<br>Up to **`6.8M lines/sec`** parse throughput | Effortlessly handles massive generated templates and monolithic files |
| **Burst Typing Resilience** | **`144,153 keystrokes/sec`**<br>`0.26 ms` post-burst completion latency | Zero editor stutter, zero dropped events, and instant completion immediately after bursts |
| **Multi-File Workspace Indexing** | **`500` files (25,000 lines) in `22.2 ms`**<br>`22,519 files/sec` throughput | Project-wide `#id` targets available across partials immediately |
| **Zero-Leak Stability** | **2,000 continuous edits** in `0.16s`<br>**`0.55 MB`** memory drift | Long-running editor sessions remain rock-solid with zero memory leaks |

> [!NOTE]
> All benchmarks were measured on Apple Silicon using production release builds. The complete 6-dimension benchmark suite can be reproduced via the [`htmx-analyzer` test harness](https://github.com/adrijshikhar/htmx-analyzer).

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
