//! Started without a terminal, rowq says so and exits instead of panicking.

use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

fn rowq_on_sample() -> Command {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_rowq"));
    cmd.arg("test_data/sample.csv")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped());
    cmd
}

fn exits_with_an_error_not_a_panic(mut cmd: Command) {
    let mut child = cmd.spawn().unwrap();
    let started = Instant::now();
    let status = loop {
        if let Some(status) = child.try_wait().unwrap() {
            break status;
        }
        if started.elapsed() > Duration::from_secs(30) {
            let _ = child.kill();
            panic!("rowq kept running without a terminal");
        }
        std::thread::sleep(Duration::from_millis(50));
    };
    let mut stderr = String::new();
    std::io::Read::read_to_string(&mut child.stderr.take().unwrap(), &mut stderr).unwrap();

    assert!(!status.success());
    assert!(!stderr.contains("panicked"), "{}", stderr);
    assert!(stderr.contains("rowq needs a terminal"), "{}", stderr);
}

#[cfg(unix)]
#[test]
fn without_a_terminal_rowq_exits_with_an_error_not_a_panic() {
    use std::os::unix::process::CommandExt;

    let mut cmd = rowq_on_sample();
    // A new session has no controlling terminal, so `/dev/tty` cannot be opened either —
    // the state of a cron job, a CI step, or a shell escape that runs without one.
    unsafe {
        cmd.pre_exec(|| {
            libc::setsid();
            Ok(())
        });
    }
    exits_with_an_error_not_a_panic(cmd);
}

#[cfg(windows)]
#[test]
fn without_a_console_rowq_exits_with_an_error_not_a_panic() {
    use std::os::windows::process::CommandExt;

    // DETACHED_PROCESS: the child gets no console at all, so there is no `CONIN$` to
    // fall back on — Windows' version of a process with no controlling terminal.
    const DETACHED_PROCESS: u32 = 0x0000_0008;
    let mut cmd = rowq_on_sample();
    cmd.creation_flags(DETACHED_PROCESS);
    exits_with_an_error_not_a_panic(cmd);
}
