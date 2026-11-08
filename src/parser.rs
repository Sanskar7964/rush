// parser.rs — Tokenise raw input into a Command struct
//
// Phase 1: naive whitespace split. We'll extend this in later phases
// to understand |, >, <, >>, and & characters.

/// A parsed command ready for execution.
///
/// # Rust note — struct
/// A struct groups related data together, like a class with only fields.
/// `#[derive(Debug)]` auto-generates a "{:?}" printer for free.
#[derive(Debug)]
pub struct Command {
    /// The program name, e.g. "ls"
    pub name: String,
    /// Arguments after the program name, e.g. ["-la", "/tmp"]
    pub args: Vec<String>,
}

/// Parse a line of raw text into a Command.
///
/// Returns None if the line is empty or only whitespace.
///
/// # Rust note — Option<T>
/// Rust has no null. Instead functions that might return "nothing" return
/// Option<T>, which is either Some(value) or None. The caller must handle both.
pub fn parse_line(line: &str) -> Option<Command> {
    // Split on whitespace, collect into a Vec of &str slices
    let mut tokens: Vec<&str> = line.split_whitespace().collect();

    if tokens.is_empty() {
        return None; // empty input — do nothing
    }

    // Remove and take the first token as the command name.
    // `remove(0)` returns the item and shifts the rest left.
    // `.to_string()` converts &str (borrowed) → String (owned). We need an
    // owned String because Command will *own* its data beyond this function.
    let name = tokens.remove(0).to_string();

    // Convert remaining &str slices to owned Strings
    let args = tokens.iter().map(|t| t.to_string()).collect();

    Some(Command { name, args })
}
