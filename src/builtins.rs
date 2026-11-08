// builtins.rs — Commands implemented inside the shell process itself
//
// Built-ins can't be external programs because they need to modify the
// shell's own state — e.g. `cd` changes the shell's working directory,
// not some child process's.

use std::env;
use std::path::Path;
use crate::parser::Command;

/// Return value from a builtin.
pub enum BuiltinResult {
    Handled,       // command was a builtin and ran
    Exit(i32),     // shell should exit with this code
    NotABuiltin,   // not a builtin, let executor handle it
}

/// Try to handle `cmd` as a builtin. Returns NotABuiltin if unrecognised.
pub fn run_builtin(cmd: &Command, last_exit: i32) -> BuiltinResult {
    match cmd.name.as_str() {
        "exit" => {
            // `exit [code]` — optional exit code, default 0
            let code = cmd.args.first()
                .and_then(|s| s.parse::<i32>().ok())
                .unwrap_or(0);
            BuiltinResult::Exit(code)
        }

        "cd" => {
            // `cd [dir]` — no arg means $HOME
            let target = match cmd.args.first() {
                Some(dir) => dir.clone(),
                None => env::var("HOME").unwrap_or_else(|_| "/".to_string()),
            };

            if let Err(e) = env::set_current_dir(Path::new(&target)) {
                eprintln!("rush: cd: {target}: {e}");
            }
            BuiltinResult::Handled
        }

        "echo" => {
            // `echo [args...]` — print args joined by spaces
            println!("{}", cmd.args.join(" "));
            BuiltinResult::Handled
        }

        "export" => {
            // `export NAME=VALUE` — set an environment variable
            for arg in &cmd.args {
                if let Some((key, val)) = arg.split_once('=') {
                    env::set_var(key, val);
                } else {
                    eprintln!("rush: export: usage: export NAME=VALUE");
                }
            }
            BuiltinResult::Handled
        }

        "pwd" => {
            match env::current_dir() {
                Ok(path) => println!("{}", path.display()),
                Err(e) => eprintln!("rush: pwd: {e}"),
            }
            BuiltinResult::Handled
        }

        "?" => {
            // Print last exit code, like `echo $?` in bash
            println!("{last_exit}");
            BuiltinResult::Handled
        }

        _ => BuiltinResult::NotABuiltin,
    }
}
