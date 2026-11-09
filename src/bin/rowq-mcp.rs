//! The MCP server on its own: `rowq-mcp` is `rowq --mcp`.
//!
//! On Windows it is built for the GUI subsystem, so a client with no console of its
//! own — Claude Desktop, an editor extension, the Codex app — starts it without a
//! console window appearing. Its stdin and stdout are the pipes the client passes,
//! as for any other server. `rowq` itself stays a console program: its interface
//! needs one.
#![cfg_attr(windows, windows_subsystem = "windows")]

use std::io::Write;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut write = false;
    let mut ignored = Vec::new();
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--mcp-write" => write = true,
            "--mcp" => {}
            _ => ignored.push(arg),
        }
    }
    if !ignored.is_empty() {
        // Started for the GUI subsystem, there may be no stderr at all; the note is
        // worth a line, not a failure.
        let _ = writeln!(
            std::io::stderr(),
            "rowq-mcp: ignoring {}; the model names the files it wants.",
            ignored.join(" ")
        );
    }
    rowq::mcp::serve(write)
}
