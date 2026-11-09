//! Paths typed into the interface: the home directory and path completion work with
//! the platform's own separators, so `~\data.csv` and `C:\data\` behave on Windows as
//! `~/data.csv` and `data/` do everywhere.

use rowq::app::{expand_tilde, split_path_input};
use std::path::Path;

#[test]
fn tilde_expands_to_the_home_directory() {
    let home = std::env::home_dir().unwrap();
    assert_eq!(expand_tilde("~"), home);
    assert_eq!(expand_tilde("~/data.csv"), home.join("data.csv"));
}

#[test]
fn a_tilde_inside_a_name_is_left_alone() {
    assert_eq!(expand_tilde("~data.csv"), Path::new("~data.csv"));
}

#[cfg(windows)]
#[test]
fn tilde_with_a_backslash_expands_on_windows() {
    let home = std::env::home_dir().unwrap();
    assert_eq!(expand_tilde(r"~\data.csv"), home.join("data.csv"));
}

#[test]
fn a_trailing_slash_completes_inside_the_directory() {
    assert_eq!(split_path_input("data/"), (Path::new("data/"), ""));
    assert_eq!(split_path_input("data/sa"), (Path::new("data"), "sa"));
}

#[test]
fn a_bare_name_completes_in_the_current_directory() {
    assert_eq!(split_path_input("sa"), (Path::new(""), "sa"));
}

#[cfg(windows)]
#[test]
fn a_trailing_backslash_completes_inside_the_directory() {
    assert_eq!(split_path_input(r"C:\data\"), (Path::new(r"C:\data\"), ""));
    assert_eq!(
        split_path_input(r"C:\data\sa"),
        (Path::new(r"C:\data"), "sa")
    );
}
