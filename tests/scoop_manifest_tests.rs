//! The Scoop manifest, mirrored into the bucket denisotree/scoop-rowq at release
//! time. It has to name the Windows zip of the version Cargo.toml carries.

use serde_json::Value;
use std::path::Path;

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

fn manifest() -> Value {
    serde_json::from_str(
        &std::fs::read_to_string(root().join("packaging/scoop/rowq.json")).unwrap(),
    )
    .unwrap()
}

fn crate_version() -> String {
    let cargo: toml_edit::DocumentMut = std::fs::read_to_string(root().join("Cargo.toml"))
        .unwrap()
        .parse()
        .unwrap();
    cargo["package"]["version"].as_str().unwrap().to_string()
}

#[test]
fn the_scoop_manifest_tracks_the_crate_version() {
    let m = manifest();
    let v = crate_version();
    assert_eq!(m["version"], v.as_str());
    assert_eq!(
        m["url"],
        format!(
            "https://github.com/denisotree/rowq/releases/download/v{v}/rowq-v{v}-x86_64-pc-windows-msvc.zip"
        )
        .as_str()
    );
    assert_eq!(
        m["extract_dir"],
        format!("rowq-v{v}-x86_64-pc-windows-msvc").as_str()
    );
}

#[test]
fn the_scoop_manifest_installs_rowq() {
    let bins = manifest()["bin"].clone();
    let bins: Vec<&str> = bins
        .as_array()
        .unwrap()
        .iter()
        .map(|b| b.as_str().unwrap())
        .collect();
    assert!(bins.contains(&"rowq.exe"), "{bins:?}");
}

#[test]
fn the_scoop_manifest_updates_from_github() {
    let m = manifest();
    assert_eq!(
        m["checkver"]["github"],
        "https://github.com/denisotree/rowq"
    );
    assert_eq!(
        m["autoupdate"]["url"],
        "https://github.com/denisotree/rowq/releases/download/v$version/rowq-v$version-x86_64-pc-windows-msvc.zip"
    );
    assert_eq!(
        m["autoupdate"]["extract_dir"],
        "rowq-v$version-x86_64-pc-windows-msvc"
    );
    let hash_url = m["autoupdate"]["hash"]["url"].as_str().unwrap();
    assert!(hash_url.ends_with("/SHA256SUMS"), "{hash_url}");
}
