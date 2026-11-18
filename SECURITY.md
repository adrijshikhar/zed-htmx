# Security Policy

## Supported Versions

Security updates are provided for the current active release line:

| Version | Supported |
| :--- | :--- |
| `0.2.x` | ✅ Yes |
| `< 0.2.0` | ❌ No |

---

## Security Architecture

- **Attestation & Hash Verification:** Prebuilt binaries for `htmx-analyzer` are verified against SHA-256 hashes compiled into `src/pinned.rs`. Hashes are verified against GitHub build provenance attestations from the upstream release workflow prior to pinning.
- **Integrity Enforcement:** Downloaded binaries must match their pinned SHA-256 digest before being made executable or launched. Any mismatch causes immediate deletion of the downloaded file.
- **PATH Security:** User-provided or PATH-installed binaries are executed directly when present, giving the operator full control over the runtime binary.

---

## Reporting a Vulnerability

If you discover a security issue or vulnerability in `zed-htmx`, please do **not** report it via public GitHub issues.

### Reporting Process

1. **GitHub Private Vulnerability Reporting (Preferred):**
   Navigate to the [Security Advisories](https://github.com/adrijshikhar/zed-htmx/security/advisories) tab and click **"Report a vulnerability"**.

2. **Email Disclosure:**
   Email the maintainer at **adrijshikhar@gmail.com** with the subject `[SECURITY] zed-htmx vulnerability report`.

### Timeline & Acknowledgments

- **Initial Response:** Within 48 hours.
- **Remediation:** Coordinated patch and release.
- **Credit:** Reporters will be acknowledged in release notes.
