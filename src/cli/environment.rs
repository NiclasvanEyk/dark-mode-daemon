use std::io::{self, IsTerminal};

pub struct Environment {
    /// Whether the current command is being piped into something.
    pub piped: bool,
}

impl Environment {
    pub fn infer() -> Self {
        let is_terminal = io::stdin().is_terminal();
        Self {
            piped: !is_terminal,
        }
    }
}
