// main.rs — Entry point for rush (Rust Shell)
//
// Rust programs always start at fn main().
// We keep main() tiny: just boot the Shell and hand over control.

mod builtins;   // src/builtins.rs
mod executor;   // src/executor.rs
mod parser;     // src/parser.rs
mod shell;      // src/shell.rs

fn main() {
    // Shell::new() sets up the readline editor and any initial state.
    // .run() starts the REPL (Read → Eval → Print → Loop).
    let mut sh = shell::Shell::new();
    sh.run();
}
