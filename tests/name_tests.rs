//! The product is rowq.  The name qrow belongs to another project (a SQL workbench with
//! its own packages), so nothing here may point at it, take it over or remove it.

use std::path::Path;

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

fn read(path: &str) -> String {
    std::fs::read_to_string(root().join(path)).unwrap()
}

const PACKAGING: [&str; 7] = [
    "Cargo.toml",
    "packaging/debian/control",
    "packaging/aur/PKGBUILD",
    "packaging/aur/PKGBUILD-bin",
    "packaging/aur/.SRCINFO",
    "packaging/aur/.SRCINFO-bin",
    "packaging/homebrew/rowq.rb",
];

/// A package of rowq that conflicts with, replaces or provides `qrow` would remove or
/// shadow the other project's package on the same machine.
#[test]
fn rowq_packages_declare_nothing_about_qrow() {
    for path in PACKAGING {
        for line in read(path).lines() {
            let l = line.to_lowercase();
            let relation = ["conflicts", "replaces", "provides", "breaks"]
                .iter()
                .any(|r| l.contains(r));
            assert!(!(relation && l.contains("qrow")), "{path}: {line}");
        }
    }
}

/// One command, `rowq`: there is no short alias in any package.  The crate also
/// builds `rowq-mcp` — the MCP server alone, for Windows clients without a console —
/// and it ships in the release archives (cargo-binstall needs every declared binary
/// there), but the system packages install `rowq` only.
#[test]
fn only_the_rowq_command_ships() {
    let manifest: toml_edit::DocumentMut = read("Cargo.toml").parse().unwrap();
    let bins: Vec<&str> = manifest["bin"]
        .as_array_of_tables()
        .unwrap()
        .iter()
        .map(|b| b["name"].as_str().unwrap())
        .collect();
    assert_eq!(bins, ["rowq", "rowq-mcp"]);
    for path in [
        "packaging/aur/PKGBUILD",
        "packaging/aur/PKGBUILD-bin",
        "packaging/homebrew/rowq.rb",
    ] {
        assert!(!read(path).contains("rowq-mcp"), "{path} installs rowq-mcp");
    }
    let deb_assets = manifest["package"]["metadata"]["deb"]["assets"].to_string();
    assert!(
        !deb_assets.contains("rowq-mcp"),
        "the .deb installs rowq-mcp"
    );
    assert!(manifest["features"].get("aliases").is_none());

    for path in [
        "packaging/aur/PKGBUILD",
        "packaging/aur/PKGBUILD-bin",
        "packaging/debian/rules",
    ] {
        assert!(!read(path).contains("ln -s"), "{path} installs an alias");
    }
    assert!(!read("packaging/homebrew/rowq.rb").contains("install_symlink"));
    assert!(
        !read("Cargo.toml").contains("link_name"),
        "the .deb ships an alias"
    );
}

/// Every tracked text file, except the changelog's account of the rename itself.
fn tracked_text_files() -> Vec<String> {
    let out = std::process::Command::new("git")
        .current_dir(root())
        .args(["ls-files", "-z"])
        .output()
        .unwrap();
    String::from_utf8(out.stdout)
        .unwrap()
        .split('\0')
        .filter(|p| !p.is_empty() && *p != "CHANGELOG.md")
        .filter(|p| std::fs::read_to_string(root().join(p)).is_ok())
        .map(str::to_string)
        .collect()
}

/// qrow.dev and the name qrow went to the other project; links to them would send
/// rowq's users there.
#[test]
fn nothing_points_at_qrow() {
    let this = "tests/name_tests.rs";
    for path in tracked_text_files() {
        if path == this {
            continue;
        }
        for (n, line) in read(&path).lines().enumerate() {
            let l = line.to_lowercase();
            assert!(
                !(l.contains("qrow") || l.contains("qrw")),
                "{path}:{}: {line}",
                n + 1
            );
        }
    }
}
