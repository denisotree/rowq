//! `rowq-mcp`: the MCP server as a binary of its own. On Windows it is built for the
//! GUI subsystem, so a client without a console — Claude Desktop, an editor extension,
//! the Codex app — starts it without a console window appearing. These tests drive it
//! the way a client does: over pipes, then by closing stdin.

use serde_json::Value;
use std::io::Write;
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

const INIT: &str = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"t","version":"0"}}}"#;
const LIST: &str = r#"{"jsonrpc":"2.0","id":2,"method":"tools/list"}"#;

fn spawn(args: &[&str], stderr: Stdio) -> Child {
    Command::new(env!("CARGO_BIN_EXE_rowq-mcp"))
        .args(args)
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(stderr)
        .spawn()
        .unwrap()
}

/// Send `input` as it stands, close stdin, and return the replies, one per line.
fn exchange(args: &[&str], stderr: Stdio, input: &str) -> Vec<Value> {
    let mut child = spawn(args, stderr);
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "exited with {}", output.status);
    String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| serde_json::from_str(l).unwrap())
        .collect()
}

fn tool_names(args: &[&str]) -> Vec<String> {
    let replies = exchange(args, Stdio::null(), &format!("{INIT}\n{LIST}\n"));
    replies[1]["result"]["tools"]
        .as_array()
        .unwrap()
        .iter()
        .map(|t| t["name"].as_str().unwrap().to_string())
        .collect()
}

#[test]
fn rowq_mcp_write_exposes_the_write_tools() {
    let read_only = tool_names(&[]);
    assert!(
        !read_only.iter().any(|n| n == "rowq_write"),
        "{read_only:?}"
    );

    let writable = tool_names(&["--mcp-write"]);
    for name in ["rowq_write", "rowq_write_apply"] {
        assert!(writable.iter().any(|n| n == name), "{name}: {writable:?}");
    }
}

/// A client may pass `--mcp` out of habit; it changes nothing.
#[test]
fn rowq_mcp_accepts_a_redundant_mcp_flag() {
    assert_eq!(tool_names(&["--mcp"]), tool_names(&[]));
}

#[test]
fn rowq_mcp_reads_a_path_with_spaces_and_cyrillic() {
    let dir = std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join("Денис dir");
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("sales data.csv");
    std::fs::copy(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("test_data/sample.csv"),
        &file,
    )
    .unwrap();

    let call = serde_json::json!({"jsonrpc": "2.0", "id": 3, "method": "tools/call",
        "params": {"name": "rowq_inspect", "arguments": {"source": file}}});
    let replies = exchange(&[], Stdio::null(), &format!("{INIT}\n{call}\n"));
    let result = &replies[1]["result"];
    assert_ne!(result["isError"], true, "{result}");
    assert_eq!(result["structuredContent"]["row_count"], 20, "{result}");
}

/// A GUI-subsystem process may be started with no stderr at all. The note about
/// ignored arguments goes there, and must not take the server down with it.
#[test]
fn rowq_mcp_survives_a_missing_stderr() {
    let replies = exchange(&["extra.csv"], Stdio::null(), &format!("{INIT}\n"));
    assert_eq!(replies.len(), 1);
    assert_eq!(replies[0]["id"], 1);
}

#[test]
fn rowq_mcp_says_which_arguments_it_ignores() {
    let mut child = spawn(&["extra.csv"], Stdio::piped());
    drop(child.stdin.take());
    let output = child.wait_with_output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("extra.csv"), "{stderr}");
}

/// The client ends the session by closing stdin; the server must go with it rather
/// than linger as an orphan.
#[test]
fn rowq_mcp_exits_when_the_client_closes_stdin() {
    let mut child = spawn(&[], Stdio::null());
    let mut stdin = child.stdin.take().unwrap();
    stdin.write_all(format!("{INIT}\n").as_bytes()).unwrap();
    drop(stdin);
    let started = Instant::now();
    loop {
        if child.try_wait().unwrap().is_some() {
            break;
        }
        if started.elapsed() > Duration::from_secs(10) {
            let _ = child.kill();
            panic!("rowq-mcp kept running after stdin closed");
        }
        std::thread::sleep(Duration::from_millis(50));
    }
}

#[test]
fn rowq_mcp_accepts_crlf_framing() {
    let replies = exchange(&[], Stdio::null(), &format!("{INIT}\r\n{LIST}\r\n"));
    let ids: Vec<u64> = replies.iter().map(|r| r["id"].as_u64().unwrap()).collect();
    assert_eq!(ids, vec![1, 2]);
    assert!(replies[1]["result"]["tools"].is_array(), "{:?}", replies[1]);
}

/// The PE header's Subsystem field: 2 is a GUI program, which Windows starts without
/// a console; 3 is a console program, which gets one — what rowq itself needs.
#[cfg(windows)]
#[test]
fn rowq_mcp_is_a_gui_subsystem_binary_and_rowq_is_not() {
    fn subsystem(exe: &str) -> u16 {
        let bytes = std::fs::read(exe).unwrap();
        let pe = u32::from_le_bytes(bytes[0x3C..0x40].try_into().unwrap()) as usize;
        assert_eq!(&bytes[pe..pe + 4], b"PE\0\0", "{exe} is not a PE file");
        let at = pe + 24 + 68;
        u16::from_le_bytes(bytes[at..at + 2].try_into().unwrap())
    }
    assert_eq!(subsystem(env!("CARGO_BIN_EXE_rowq-mcp")), 2);
    assert_eq!(subsystem(env!("CARGO_BIN_EXE_rowq")), 3);
}
