## Description

<!-- Describe the changes introduced by this pull request. -->

## Related Issues

<!-- Closes #123, Fixes #456 -->

## Checklist

- [ ] I have read the [CONTRIBUTING.md](CONTRIBUTING.md) guide.
- [ ] Code follows `#![forbid(unsafe_code)]`.
- [ ] Verified `cargo clippy --target wasm32-wasip1 --release -- -D warnings` passes.
- [ ] Verified `cargo clippy --target wasm32-wasip2 --release -- -D warnings` passes.
- [ ] Verified `cargo fmt --all --check` produces no formatting diff.
- [ ] Verified `python3 tests/e2e_lsp_test.py` passes 100%.
- [ ] Tested loading extension in Zed locally via `zed: install dev extension`.
