// executor.rs — Run a parsed Command as a child process
//
// Phase 1 uses std::process::Command which wraps fork()+exec() for us.
// In Phase 3 (pipes) we'll drop down to raw nix syscalls.

use std::process;
use crate::parser::Command;

/// Exit status after running a command.
pub enum ExitStatus {
    Success,
    Failed(i32), // non-zero exit code
    NotFound,    // binary not in PATH
}

/// Spawn the command as a child process and wait for it to finish.
///
/// # Rust note — & vs owned
/// We borrow `cmd` with `&Command` — we don't need to own it, just read it.
/// Borrowing is cheaper (no copy) and signals "I won't modify or store this".
pub fn execute(cmd: &Command) -> ExitStatus {
    // std::process::Command is Rust's stdlib process builder.
    // Note: this shadows our own `Command` type — we use the full path here
    // to disambiguate.
    let result = process::Command::new(&cmd.name)
        .args(&cmd.args)
        .spawn(); // fork() + exec() under the hood

    match result {
        // Child process launched — wait for it to finish
        Ok(mut child) => {
            // .wait() blocks until the child exits (like waitpid in C)
            match child.wait() {
                Ok(status) => {
                    if status.success() {
                        ExitStatus::Success
                    } else {
                        // .code() returns Option<i32> — None if killed by signal
                        ExitStatus::Failed(status.code().unwrap_or(1))
                    }
                }
                Err(e) => {
                    eprintln!("rush: wait error: {e}");
                    ExitStatus::Failed(1)
                }
            }
        }
        // Command not found or permission denied
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => ExitStatus::NotFound,
        Err(e) => {
            eprintln!("rush: {e}");
            ExitStatus::Failed(1)
        }
    }
}
