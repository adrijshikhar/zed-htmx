use zed_extension_api::{Architecture, Os};

pub fn platform_key(os: Os, arch: Architecture) -> Option<&'static str> {
    match (os, arch) {
        (Os::Mac, Architecture::Aarch64) => Some("darwin-arm64"),
        (Os::Mac, Architecture::X8664) => Some("darwin-x64"),
        (Os::Linux, Architecture::X8664) => Some("linux-x64"),
        (Os::Linux, Architecture::Aarch64) => Some("linux-arm64"),
        (Os::Windows, Architecture::X8664) => Some("windows-x64"),
        (Os::Windows, Architecture::Aarch64) => Some("windows-arm64"),
        _ => None,
    }
}

pub fn pinned_asset(key: &str) -> Option<(&'static str, &'static str)> {
    crate::pinned::ASSETS
        .iter()
        .find(|(k, _, _)| *k == key)
        .map(|(_, name, sha)| (*name, *sha))
}

pub fn sha256_file(path: &str) -> Result<String, String> {
    use sha2::{Digest, Sha256};
    use std::fmt::Write as _;
    use std::io::Read;

    let mut file = std::fs::File::open(path).map_err(|e| format!("failed to open {path}: {e}"))?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];
    loop {
        let count = file
            .read(&mut buffer)
            .map_err(|e| format!("failed to read {path}: {e}"))?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }
    let hash = hasher.finalize();
    let mut hex = String::with_capacity(hash.len() * 2);
    for byte in hash {
        let _ = write!(hex, "{byte:02x}");
    }
    Ok(hex)
}

pub fn verify_download(path: &str, expected_hex: &str) -> Result<(), String> {
    let actual_hex = match sha256_file(path) {
        Ok(hash) => hash,
        Err(e) => {
            let _ = std::fs::remove_file(path);
            return Err(e);
        }
    };

    if !actual_hex.eq_ignore_ascii_case(expected_hex) {
        let _ = std::fs::remove_file(path);
        return Err(format!(
            "htmx-analyzer download failed verification: expected {expected_hex}, got {actual_hex}"
        ));
    }

    Ok(())
}

pub fn is_stale_install_dir(name: &str, keep: &str) -> bool {
    name.starts_with("htmx-analyzer-") && name != keep
}

pub fn unsupported_platform_message(os: Os, arch: Architecture) -> String {
    let os_str = match os {
        Os::Mac => "mac",
        Os::Linux => "linux",
        Os::Windows => "windows",
    };
    let arch_str = match arch {
        Architecture::Aarch64 => "aarch64",
        Architecture::X8664 => "x86_64",
        Architecture::X86 => "x86",
    };
    format!(
        "htmx-analyzer does not publish prebuilt binaries for {os_str}-{arch_str}. Please install htmx-analyzer on your PATH."
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn platform_keys_cover_six_platforms() {
        assert_eq!(
            platform_key(Os::Mac, Architecture::Aarch64),
            Some("darwin-arm64")
        );
        assert_eq!(
            platform_key(Os::Mac, Architecture::X8664),
            Some("darwin-x64")
        );
        assert_eq!(
            platform_key(Os::Linux, Architecture::X8664),
            Some("linux-x64")
        );
        assert_eq!(
            platform_key(Os::Linux, Architecture::Aarch64),
            Some("linux-arm64")
        );
        assert_eq!(
            platform_key(Os::Windows, Architecture::X8664),
            Some("windows-x64")
        );
        assert_eq!(
            platform_key(Os::Windows, Architecture::Aarch64),
            Some("windows-arm64")
        );
        assert_eq!(platform_key(Os::Linux, Architecture::X86), None);
    }

    #[test]
    fn pinned_assets_are_well_formed() {
        let mut keys = HashSet::new();
        for (key, asset, sha) in crate::pinned::ASSETS {
            assert!(keys.insert(*key), "duplicate key in ASSETS: {key}");
            assert!(
                asset.starts_with("htmx-analyzer-"),
                "asset {asset} does not start with htmx-analyzer-"
            );
            assert_eq!(sha.len(), 64, "sha for {asset} must be 64 characters");
            assert!(
                sha.chars()
                    .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()),
                "sha for {asset} must be lowercase hex"
            );
        }
    }

    #[test]
    fn sha256_file_matches_known_vector() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("zed_htmx_test_vector.txt");
        std::fs::write(&test_file, b"abc").expect("write test file");
        let hash = sha256_file(test_file.to_str().unwrap()).expect("sha256 calculation");
        std::fs::remove_file(&test_file).ok();
        assert_eq!(
            hash,
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn verify_download_deletes_on_mismatch() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("zed_htmx_test_mismatch.txt");
        std::fs::write(&test_file, b"abc").expect("write test file");
        let result = verify_download(
            test_file.to_str().unwrap(),
            "0000000000000000000000000000000000000000000000000000000000000000",
        );
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err.contains("expected"),
            "error should contain 'expected': {err}"
        );
        assert!(!test_file.exists(), "file should be deleted on mismatch");
    }

    #[test]
    fn stale_dir_detection() {
        assert!(is_stale_install_dir(
            "htmx-analyzer-v0.1.0",
            "htmx-analyzer-v0.2.0"
        ));
        assert!(!is_stale_install_dir(
            "htmx-analyzer-v0.2.0",
            "htmx-analyzer-v0.2.0"
        ));
        assert!(!is_stale_install_dir(
            concat!("htmx-", "lsp-x"),
            "htmx-analyzer-v0.2.0"
        ));
        assert!(!is_stale_install_dir("cache", "htmx-analyzer-v0.2.0"));
    }

    #[test]
    fn unsupported_platform_message() {
        let msg = super::unsupported_platform_message(Os::Linux, Architecture::X86);
        assert!(
            msg.contains("linux-x86"),
            "message should contain linux-x86: {msg}"
        );
        assert!(
            msg.contains("install htmx-analyzer on your PATH"),
            "message should tell user to install on PATH: {msg}"
        );
    }

    #[test]
    fn pinned_assets_cover_all_platforms() {
        assert_eq!(crate::pinned::SERVER_TAG, "v0.2.0");
        let supported = [
            (Os::Mac, Architecture::Aarch64),
            (Os::Mac, Architecture::X8664),
            (Os::Linux, Architecture::X8664),
            (Os::Linux, Architecture::Aarch64),
            (Os::Windows, Architecture::X8664),
            (Os::Windows, Architecture::Aarch64),
        ];
        for (os, arch) in supported {
            let key = platform_key(os, arch).expect("supported platform must have key");
            assert!(
                pinned_asset(key).is_some(),
                "missing pinned asset for platform key: {key}"
            );
        }
    }
}
