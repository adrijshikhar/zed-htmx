# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-11-18

### Changed
- **Breaking:** Renamed extension ID to `htmx` and language server key to `htmx-analyzer`. User settings in Zed are now located under `lsp.htmx-analyzer`.
- Attached `htmx-analyzer` to ten languages: HTML, Jinja2, Django, PHP, HTML+ERB, Blade, Vue.js, Astro, Svelte, and Templ.
- Replaced dynamic GitHub "latest" release downloads with a pinned binary installation model. Release artifacts and SHA-256 digests are pinned in the extension and verified upon download.
- Removed deprecated `language` key in `extension.toml`.
- Dropped all lookups and fallbacks for unrelated `htmx-lsp` binaries.

### Added
- Integrity verification checking SHA-256 hashes of downloaded binaries before execution, automatically deleting mismatched downloads.
- Automated cleanup of stale `htmx-analyzer-*` installation directories.
- Host-level integration test suite for `extension.toml` manifest validation.
- Dependabot configuration for Cargo and GitHub Actions dependencies.
- Host test job in CI workflow.

## [0.1.0] - 2026-11-18

### Added
- Initial release of the Zed HTMX extension.
