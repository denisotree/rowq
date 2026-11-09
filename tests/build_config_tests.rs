//! How the crate builds without the bundled DuckDB: on docs.rs, and against a system
//! libduckdb as the README describes.

use std::process::Command;

fn manifest_dir() -> &'static str {
    env!("CARGO_MANIFEST_DIR")
}

/// The flags docs.rs builds with, read from `[package.metadata.docs.rs]`.
fn docs_rs_flags() -> Vec<String> {
    let text = std::fs::read_to_string(format!("{}/Cargo.toml", manifest_dir())).unwrap();
    let doc: toml_edit::DocumentMut = text.parse().unwrap();
    let meta = &doc["package"]["metadata"]["docs"]["rs"];
    let mut flags = Vec::new();
    if meta["no-default-features"].as_bool() == Some(true) {
        flags.push("--no-default-features".to_string());
    }
    let features: Vec<&str> = meta
        .get("features")
        .and_then(|f| f.as_array())
        .map(|a| a.iter().filter_map(|v| v.as_str()).collect())
        .unwrap_or_default();
    if !features.is_empty() {
        flags.push(format!("--features={}", features.join(",")));
    }
    flags
}

/// `cargo tree` over the Linux build with `flags`, extra arguments appended.
fn cargo_tree(flags: &[String], extra: &[&str]) -> (bool, String) {
    let out = Command::new(env!("CARGO"))
        .current_dir(manifest_dir())
        .args(["tree", "--locked", "--target", "x86_64-unknown-linux-gnu"])
        .args(flags)
        .args(extra)
        .output()
        .unwrap();
    let text =
        String::from_utf8_lossy(&out.stdout).to_string() + &String::from_utf8_lossy(&out.stderr);
    (out.status.success(), text)
}

/// docs.rs builds without network.  An HTTP client in a build script means a build
/// script that downloads — libduckdb-sys's `download-lib` fetches a pre-built libduckdb
/// and panics when it cannot, which failed the docs.rs build of 1.1.0.
#[test]
fn the_docs_rs_build_downloads_nothing() {
    let (ok, text) = cargo_tree(&docs_rs_flags(), &["-e", "normal,build", "-i", "ureq"]);
    let absent = !ok && text.contains("did not match any packages");
    assert!(
        absent || (ok && text.trim().is_empty()),
        "a build script in the docs.rs build can download:\n{text}"
    );
}

/// Without the bundled DuckDB, a system libduckdb is found through pkg-config:
/// Homebrew on Apple Silicon puts it in /opt/homebrew/lib, off the linker's path.
#[test]
fn the_system_duckdb_build_finds_libduckdb_through_pkg_config() {
    let flags = vec!["--no-default-features".to_string()];
    let (ok, text) = cargo_tree(&flags, &["-e", "features", "-i", "libduckdb-sys"]);
    assert!(ok, "{text}");
    assert!(
        text.contains("libduckdb-sys feature \"pkg-config\""),
        "libduckdb-sys is built without pkg-config:\n{text}"
    );
}
