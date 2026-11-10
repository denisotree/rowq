//! The external editor (`E`): which program and arguments `$EDITOR` names.

use rowq::app::editor_command;

fn cmd(editor: &str) -> (String, Vec<String>) {
    editor_command(editor)
}

#[test]
fn a_bare_editor_name_runs_as_it_is() {
    assert_eq!(cmd("vi"), ("vi".to_string(), vec![]));
}

/// `EDITOR="code --wait"` is how a GUI editor is made to block; the flag is an
/// argument, not part of the program's name.
#[test]
fn an_editor_with_arguments_is_split_into_program_and_arguments() {
    assert_eq!(
        cmd("code --wait"),
        ("code".to_string(), vec!["--wait".to_string()])
    );
}

/// A quoted path with spaces, then the editor's own flags.
#[test]
fn a_quoted_program_keeps_its_spaces() {
    assert_eq!(
        cmd(r#""/Applications/Sublime Text.app/Contents/SharedSupport/bin/subl" -n -w"#),
        (
            "/Applications/Sublime Text.app/Contents/SharedSupport/bin/subl".to_string(),
            vec!["-n".to_string(), "-w".to_string()]
        )
    );
}

/// An unquoted path that names a real file is one program, spaces and all.
#[test]
fn an_existing_path_with_spaces_is_one_program() {
    let dir = std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join("my editor");
    std::fs::create_dir_all(&dir).unwrap();
    let exe = dir.join("ed it");
    std::fs::write(&exe, "").unwrap();
    let path = exe.to_string_lossy().into_owned();
    assert_eq!(cmd(&path), (path.clone(), vec![]));
}
