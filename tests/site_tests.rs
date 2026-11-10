//! rowq.dev is the landing page, served from its own repository (denisotree/rowq.dev).
//! This repository publishes only the APT repository, from gh-pages, at apt.rowq.dev.

use std::path::Path;

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

fn read(path: &str) -> String {
    std::fs::read_to_string(root().join(path)).unwrap()
}

/// The install notes point APT at apt.rowq.dev, the key included.
#[test]
fn apt_is_installed_from_apt_rowq_dev() {
    let readme = read("README.md");
    assert!(
        readme.contains("https://apt.rowq.dev/rowq.gpg"),
        "README: key URL"
    );
    assert!(
        readme.contains("https://apt.rowq.dev stable main"),
        "README: sources line"
    );
    for path in [
        "README.md",
        ".github/workflows/apt.yml",
        "packaging/apt/publish.sh",
    ] {
        assert!(
            !read(path).contains("rowq.dev/apt"),
            "{path} still uses rowq.dev/apt"
        );
    }
}

/// gh-pages here carries the APT repository at its root under the apt.rowq.dev domain;
/// the landing page is not copied in from this repository any more.
#[test]
fn gh_pages_carries_only_the_apt_repository() {
    assert!(!root().join("packaging/site").exists());
    let workflow = read(".github/workflows/apt.yml");
    assert!(!workflow.contains("packaging/site"));
    assert!(
        workflow.contains("apt.rowq.dev"),
        "apt.yml does not set the domain"
    );
}

/// Which editor `E` opens, and how to give it flags.
#[test]
fn the_keybindings_explain_the_external_editor() {
    for path in ["docs/en/keybindings.md", "docs/ru/keybindings.md"] {
        let text = read(path);
        for needle in ["`$VISUAL`", "EDITOR=\"code --wait\""] {
            assert!(text.contains(needle), "{path}: no {needle:?}");
        }
    }
}

/// There is no alias any more; the install notes must not promise one is left out.
#[test]
fn the_install_notes_do_not_mention_an_alias() {
    for path in [
        "README.md",
        "docs/en/getting-started.md",
        "docs/ru/getting-started.md",
    ] {
        let text = read(path);
        for stale in ["without the\nalias", "without the alias", "без псевдонима"] {
            assert!(!text.contains(stale), "{path}: {stale:?}");
        }
    }
}

/// rowq runs on Linux and macOS; on Windows, through WSL like any Linux program. The
/// user docs say nothing about a native Windows build there is not.
#[test]
fn the_docs_do_not_describe_native_windows() {
    let mut paths = vec!["README.md".to_string()];
    for lang in ["en", "ru"] {
        for entry in std::fs::read_dir(root().join("docs").join(lang)).unwrap() {
            let p = entry.unwrap().path();
            if p.extension().is_some_and(|e| e == "md") {
                paths.push(
                    p.strip_prefix(root())
                        .unwrap()
                        .to_string_lossy()
                        .into_owned(),
                );
            }
        }
    }
    for path in paths {
        for (n, line) in read(&path).lines().enumerate() {
            assert!(!line.contains("Windows"), "{path}:{}: {line}", n + 1);
        }
    }
}

/// publish.sh builds the index with apt-ftparchive, from apt-utils. The runner image
/// stopped carrying it (the v1.0.1 publish failed with "command not found"), so the
/// workflow installs it rather than counting on the image.
#[test]
fn the_apt_workflow_installs_apt_ftparchive_before_publishing() {
    let workflow = read(".github/workflows/apt.yml");
    let install = workflow
        .find("apt-get install -y --no-install-recommends apt-utils")
        .expect("apt.yml does not install apt-utils");
    let publish = workflow.find("packaging/apt/publish.sh").unwrap();
    assert!(
        install < publish,
        "apt-utils is installed after publish.sh runs"
    );
}
