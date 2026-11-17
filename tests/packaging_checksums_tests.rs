//! The checksums in the packaging files are filled in by hand during a release. Each one
//! is either the placeholder the release resets it to or a real SHA-256 — never empty,
//! which is what v1.0.3 briefly shipped to AUR and the tap when `SHA256SUMS` was read
//! before the release had it.

fn read(path: &str) -> String {
    std::fs::read_to_string(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path)).unwrap()
}

/// The value of every `sha256…` field in `path`, quotes and brackets stripped.
fn checksums(path: &str) -> Vec<String> {
    read(path)
        .lines()
        .map(str::trim)
        .filter(|l| l.starts_with("sha256"))
        .map(|l| {
            let value = match l.split_once('=') {
                Some((_, v)) => v,
                None => l.trim_start_matches("sha256"),
            };
            value
                .trim()
                .trim_matches(|c| matches!(c, '(' | ')' | '\'' | '"'))
                .to_string()
        })
        .collect()
}

#[test]
fn every_checksum_is_a_placeholder_or_a_sha256() {
    for (path, count) in [
        ("packaging/homebrew/rowq.rb", 4),
        ("packaging/aur/PKGBUILD", 1),
        ("packaging/aur/PKGBUILD-bin", 2),
        ("packaging/aur/.SRCINFO", 1),
        ("packaging/aur/.SRCINFO-bin", 2),
    ] {
        let found = checksums(path);
        assert_eq!(found.len(), count, "{path}: {found:?}");
        for value in found {
            let real = value.len() == 64
                && value
                    .bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b));
            assert!(
                real || value == "SKIP" || value == "FILL_IN_AFTER_RELEASE",
                "{path}: checksum {value:?}"
            );
        }
    }
}
