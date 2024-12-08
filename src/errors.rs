/*!
 * errors.rs
 *
 * Defines error types for lexing, parsing, and runtime.
 * Each error type is simple and just carries a string message.
 */

use std::fmt;

/// Error type for lexing
pub struct LexError {
    msg: String,
}

impl LexError {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LexError: {}", self.msg)
    }
}

impl fmt::Debug for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LexError: {}", self.msg)
    }
}

/// Error type for parsing
pub struct ParseError {
    msg: String,
}

impl ParseError {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseError: {}", self.msg)
    }
}

impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseError: {}", self.msg)
    }
}

/// Error type for runtime
pub struct RuntimeError {
    msg: String,
}

impl RuntimeError {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RuntimeError: {}", self.msg)
    }
}

impl fmt::Debug for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RuntimeError: {}", self.msg)
    }
}
