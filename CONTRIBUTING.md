# Contributing to zed-htmx

Thank you for your interest in contributing to **zed-htmx**! This extension connects Zed with [`htmx-analyzer`](https://github.com/adrijshikhar/htmx-analyzer) to provide completion, hover documentation, diagnostics, and quick-fixes for HTMX 2.0.

---

## Development Prerequisites

- **Rust toolchain:** Latest stable Rust (`rustup default stable`).
- **WebAssembly target:**
  ```bash
  rustup target add wasm32-wasip1
  ```
- **Wasm Tools (Component Model):** Required for building modern Zed 0.130+ WebAssembly components:
  ```bash
  cargo install wasm-tools
  ```
- **Zed Editor:** Installed locally for dev extension testing.
- **Python 3:** For running the end-to-end integration test suite.

---

## Project Structure

```text
zed-htmx/
├── Cargo.toml          # Rust crate configuration (cdylib, zed_extension_api dependency)
├── extension.toml      # Zed extension manifest (id, languages, language_servers)
├── extension.wasm      # Built WebAssembly component loaded by Zed
├── src/
│   └── lib.rs          # Extension implementation: binary locator, download, and fallback logic
└── tests/
    ├── e2e_lsp_test.py # End-to-end LSP protocol verification test suite
    └── test.html       # HTML fixture for protocol testing
```

---

## Building & Packaging the Extension

1. **Compile to WebAssembly (`wasm32-wasip1`):**
   ```bash
   cargo build --target wasm32-wasip1 --release
   ```

2. **Convert to Zed Component Model:**
   ```bash
   wasm-tools component new ./target/wasm32-wasip1/release/zed_htmx.wasm \
     --adapt wasi_snapshot_preview1=wasi_snapshot_preview1.reactor.wasm \
     -o extension.wasm
   ```
   *(A copy of `wasi_snapshot_preview1.reactor.wasm` can be obtained from the [bytecodealliance/wasmtime releases](https://github.com/bytecodealliance/wasmtime/releases)).*

---

## Testing in Zed Locally

1. Open Zed.
2. Open the Command Palette (`Cmd+Shift+P` on macOS, `Ctrl+Shift+P` on Linux/Windows).
3. Search for and run **`zed: install dev extension`**.
4. Select your local `zed-htmx` repository folder.
5. Open an HTML file with HTMX attributes to verify that completions and diagnostics work.

---

## Running Automated Tests

1. **Rustfmt & Clippy checks:**
   ```bash
   cargo fmt --all --check
   cargo clippy --target wasm32-wasip1 --release -- -D warnings
   cargo clippy --target wasm32-wasip2 --release -- -D warnings
   ```

2. **E2E Protocol Verification Suite:**
   Make sure `htmx-analyzer` is installed on your `PATH`, then run:
   ```bash
   python3 tests/e2e_lsp_test.py
   ```

---

## Code Standards

- `#![forbid(unsafe_code)]` is strictly enforced.
- Keep the binary fallback and auto-download logic resilient to network errors.
- Ensure cross-platform compatibility across macOS, Linux, and Windows.
- Follow conventional commits (`feat: ...`, `fix: ...`, `refactor: ...`).
