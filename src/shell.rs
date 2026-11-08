// shell.rs — The Shell struct: state + the REPL loop
//
// REPL = Read → Evaluate → Print → Loop
// This is the heart of the shell.

use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

use crate::builtins::{self, BuiltinResult};
use crate::executor::{self, ExitStatus};
use crate::parser;

/// All mutable state the shell needs to track.
pub struct Shell {
    /// rustyline editor — handles input, history, later tab completion
    editor: DefaultEditor,
    /// Exit code of the last command (like $? in bash)
    last_exit: i32,
}

impl Shell {
    /// Construct a new Shell with defaults.
    ///
    /// # Rust note — impl block
    /// Methods live inside `impl TypeName { }` blocks.
    /// `pub fn new() -> Shell` is a "constructor" by convention (Rust has no
    /// special constructor syntax).
    pub fn new() -> Shell {
        // DefaultEditor::new() returns a Result — we unwrap() here because
        // if readline can't init, there's no point continuing.
        let editor = DefaultEditor::new().expect("Failed to init readline editor");

        Shell {
            editor,
            last_exit: 0,
        }
    }

    /// Start the REPL. Runs until the user types `exit` or presses Ctrl-D.
    pub fn run(&mut self) {
        loop {
            // ── READ ──────────────────────────────────────────────────────
            let prompt = self.build_prompt();

            // readline() reads a line, handles Ctrl-C / Ctrl-D, keeps history
            let line = match self.editor.readline(&prompt) {
                Ok(line) => line,
                Err(ReadlineError::Interrupted) => {
                    // Ctrl-C — clear the line and loop (like real shells)
                    println!();
                    continue;
                }
                Err(ReadlineError::Eof) => {
                    // Ctrl-D — exit gracefully
                    println!("exit");
                    break;
                }
                Err(e) => {
                    eprintln!("rush: readline error: {e}");
                    break;
                }
            };

            // Add non-empty lines to history (up-arrow recall)
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            let _ = self.editor.add_history_entry(trimmed);

            // ── EVALUATE ─────────────────────────────────────────────────
            self.evaluate(trimmed);
        }
    }

    /// Parse and execute one line of input.
    fn evaluate(&mut self, line: &str) {
        // Parse the raw string into a Command (or None if blank)
        let cmd = match parser::parse_line(line) {
            Some(c) => c,
            None => return,
        };

        // Try builtins first — they need to run inside the shell process
        match builtins::run_builtin(&cmd, self.last_exit) {
            BuiltinResult::Handled => {
                self.last_exit = 0;
                return;
            }
            BuiltinResult::Exit(code) => {
                std::process::exit(code);
            }
            BuiltinResult::NotABuiltin => {
                // Fall through to executor
            }
        }

        // Spawn an external process
        match executor::execute(&cmd) {
            ExitStatus::Success => self.last_exit = 0,
            ExitStatus::Failed(code) => {
                self.last_exit = code;
            }
            ExitStatus::NotFound => {
                eprintln!("rush: {}: command not found", cmd.name);
                self.last_exit = 127; // 127 = standard "not found" code
            }
        }
    }

    /// Build the prompt string shown before each input line.
    fn build_prompt(&self) -> String {
        // Show current directory (basename only) + $ indicator
        // In a later phase we can add git branch, exit code colour, etc.
        let cwd = std::env::current_dir()
            .map(|p| {
                p.file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| "/".to_string())
            })
            .unwrap_or_else(|_| "?".to_string());

        // Colour the prompt: green if last exit was 0, red otherwise
        let indicator = if self.last_exit == 0 {
            "\x1b[32m$\x1b[0m" // green $
        } else {
            "\x1b[31m$\x1b[0m" // red $
        };

        format!("rush {cwd} {indicator} ")
    }
}
