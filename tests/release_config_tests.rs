//! The release workflow: a Windows zip beside the tarballs, the static CRT for that
//! build only, `rowq-mcp` in every archive, and a manual run that only builds.

use serde_yaml_ng::Value;

fn workflow() -> Value {
    let path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(".github/workflows/release.yml");
    serde_yaml_ng::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
}

fn matrix() -> Vec<Value> {
    workflow()["jobs"]["build"]["strategy"]["matrix"]["include"]
        .as_sequence()
        .unwrap()
        .clone()
}

/// A build step's script, with the workflow's `BIN_NAME` substituted in.
fn step_run(name: &str) -> String {
    let wf = workflow();
    let bin = wf["env"]["BIN_NAME"].as_str().unwrap().to_string();
    wf["jobs"]["build"]["steps"]
        .as_sequence()
        .unwrap()
        .iter()
        .find(|s| s["name"].as_str() == Some(name))
        .unwrap_or_else(|| panic!("no step named {name:?}"))["run"]
        .as_str()
        .unwrap()
        .replace("${BIN_NAME}", &bin)
        .replace("$env:BIN_NAME", &bin)
}

fn str_of<'a>(entry: &'a Value, key: &str) -> &'a str {
    entry[key].as_str().unwrap_or("")
}

#[test]
fn the_release_builds_a_windows_zip() {
    let windows = matrix()
        .into_iter()
        .find(|e| str_of(e, "target") == "x86_64-pc-windows-msvc")
        .expect("no Windows build in the release matrix");
    assert_eq!(str_of(&windows, "os"), "windows-latest");
    assert_eq!(str_of(&windows, "archive"), "zip");
    assert!(str_of(&windows, "rustflags").contains("+crt-static"));
}

/// The static CRT keeps the Windows .exe free of the VC++ runtime; on Unix the
/// flag means something else (a static glibc), which nobody asked for.
#[test]
fn crt_static_is_windows_only() {
    for entry in matrix() {
        if str_of(&entry, "target") != "x86_64-pc-windows-msvc" {
            assert!(
                !str_of(&entry, "rustflags").contains("crt-static"),
                "{}",
                str_of(&entry, "target")
            );
        }
    }
}

/// cargo-binstall looks for every binary the crate declares, so every archive
/// carries `rowq-mcp`, not only the Windows one.
#[test]
fn every_archive_carries_rowq_mcp() {
    for step in ["Package (Unix)", "Package (Windows)"] {
        assert!(step_run(step).contains("rowq-mcp"), "{step}");
    }
}

/// A manual run builds every target and stops there: no GitHub Release, and no
/// push to the APT repository at apt.rowq.dev.
#[test]
fn a_manual_run_only_builds() {
    let wf = workflow();
    assert!(
        wf["on"].get("workflow_dispatch").is_some(),
        "no workflow_dispatch trigger"
    );
    for job in ["release", "apt"] {
        let cond = wf["jobs"][job]["if"].as_str().unwrap_or("");
        assert!(
            cond.contains("github.event_name == 'push'"),
            "{job}: {cond:?}"
        );
    }
}
