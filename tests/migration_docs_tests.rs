//! The commands the migration notes tell a tuitab user to paste.  They run on
//! machines set up by the old docs, so each has to work from any of the states those
//! docs left behind.

fn read(path: &str) -> String {
    std::fs::read_to_string(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path)).unwrap()
}

const NOTES: [&str; 2] = ["README.md", "CHANGELOG.md"];

/// The old docs registered the server in local scope; `remove -s user` fails there,
/// and in `remove && add` the rowq server is then never added.
#[test]
fn removing_the_old_mcp_server_does_not_name_a_scope() {
    for path in NOTES {
        let text = read(path);
        assert!(
            !text.contains("claude mcp remove -s"),
            "{path} removes the tuitab server from one scope only"
        );
    }
}

/// Homebrew renames a redirected tap by itself, and untapping a tap that holds the
/// installed formula uninstalls it.
#[test]
fn the_notes_do_not_untap_the_old_tap() {
    for path in NOTES {
        assert!(
            !read(path).contains("brew untap denisotree/tuitab"),
            "{path} tells the user to untap the tap rowq was installed from"
        );
    }
}

/// tuitab stays available at 0.11.1 as its own product, so the notes describe rowq as
/// something installed next to it, not over it.
#[test]
fn the_notes_install_rowq_beside_tuitab() {
    for path in NOTES {
        let text = read(path);
        assert!(
            text.contains("side by side"),
            "{path} does not say both can stay"
        );
        assert!(
            !text.contains("removes `tuitab`") && !text.contains("replaces tuitab"),
            "{path} still says rowq takes tuitab's place"
        );
    }
}

/// rowq has its own tap; the renamed-tap path (formula_renames, brew trust) is gone.
#[test]
fn the_homebrew_note_installs_from_the_rowq_tap() {
    for path in NOTES {
        let text = read(path);
        assert!(
            text.contains("brew install denisotree/rowq/rowq"),
            "{path} has no Homebrew command for rowq"
        );
        assert!(!text.contains("brew trust denisotree/rowq"), "{path}");
    }
}

/// Installing rowq must not remove tuitab: the binaries differ, so no package
/// declares a conflict with it or replaces it.
#[test]
fn rowq_packages_do_not_replace_tuitab() {
    for path in [
        "Cargo.toml",
        "packaging/debian/control",
        "packaging/aur/PKGBUILD",
        "packaging/aur/PKGBUILD-bin",
        "packaging/aur/.SRCINFO",
        "packaging/aur/.SRCINFO-bin",
    ] {
        for line in read(path).lines() {
            let l = line.to_lowercase();
            assert!(
                !((l.contains("conflicts") || l.contains("replaces")) && l.contains("tuitab")),
                "{path}: {line}"
            );
        }
    }
    assert!(!std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("packaging/homebrew/formula_renames.json")
        .exists());
}
