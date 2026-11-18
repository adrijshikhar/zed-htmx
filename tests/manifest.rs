use std::collections::HashSet;

fn load_manifest() -> toml::Table {
    let manifest_path = concat!(env!("CARGO_MANIFEST_DIR"), "/extension.toml");
    let content = std::fs::read_to_string(manifest_path).expect("failed to read extension.toml");
    content
        .parse::<toml::Table>()
        .expect("failed to parse extension.toml")
}

#[test]
fn manifest_id_and_server() {
    let manifest = load_manifest();
    assert_eq!(manifest.get("id").and_then(|v| v.as_str()), Some("htmx"));

    let servers = manifest
        .get("language_servers")
        .and_then(|v| v.as_table())
        .expect("language_servers table missing");
    let keys: Vec<&str> = servers.keys().map(|k| k.as_str()).collect();
    assert_eq!(keys, vec!["htmx-analyzer"]);

    let server = servers.get("htmx-analyzer").unwrap().as_table().unwrap();
    assert_eq!(
        server.get("name").and_then(|v| v.as_str()),
        Some("htmx-analyzer")
    );
}

#[test]
fn manifest_languages() {
    let manifest = load_manifest();
    let expected_languages = [
        "HTML", "Jinja2", "Django", "PHP", "HTML+ERB", "Blade", "Vue.js", "Astro", "Svelte",
        "Templ",
    ];

    let server = manifest
        .get("language_servers")
        .and_then(|v| v.as_table())
        .and_then(|s| s.get("htmx-analyzer"))
        .and_then(|v| v.as_table())
        .expect("htmx-analyzer table missing");

    assert!(
        server.get("language").is_none(),
        "deprecated 'language' key found in server table"
    );

    let languages: Vec<&str> = server
        .get("languages")
        .and_then(|v| v.as_array())
        .expect("languages array missing")
        .iter()
        .map(|v| v.as_str().expect("language entry must be string"))
        .collect();
    assert_eq!(languages, expected_languages);

    let language_ids = server
        .get("language_ids")
        .and_then(|v| v.as_table())
        .expect("language_ids table missing");

    let id_keys: HashSet<&str> = language_ids.keys().map(|k| k.as_str()).collect();
    let expected_set: HashSet<&str> = expected_languages.iter().copied().collect();
    assert_eq!(id_keys, expected_set);
}

#[test]
fn manifest_version_matches_cargo() {
    let manifest = load_manifest();
    let manifest_version = manifest
        .get("version")
        .and_then(|v| v.as_str())
        .expect("version string missing");
    assert_eq!(manifest_version, env!("CARGO_PKG_VERSION"));
    assert_eq!(env!("CARGO_PKG_VERSION"), "0.2.0");
}
