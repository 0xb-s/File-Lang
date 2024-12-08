/*!
 * tokens.rs
 *
 * Defines the Token and TokenKind types used by the lexer and parser.
 */

/// A token consists of a kind and a position.
pub struct Token {
    /// The kind of the token.
    pub kind: TokenKind,
    /// The position in the input stream (for error messages).
    pub pos: usize,
}

impl Token {
    /// Create a new token.
    pub fn new(kind: TokenKind, pos: usize) -> Self {
        Self { kind, pos }
    }
}

/// The kind of token. Some tokens carry values (like strings or identifiers).
#[derive(Clone, Debug)]
pub enum TokenKind {
    // Keywords
    Open,
    Read,
    Write,
    Append,
    Close,
    Show,
    Exit,
    As,
    Truncate,
    Search,
    Replace,
    LineCount,
    Copy,
    Move,
    Remove,
    Rename,
    ListDir,
    DumpEnv,
    Help,

    // Values
    Identifier(String),
    String(String),

    // End of statement
    EndOfStatement,
}

impl TokenKind {
    /// Check if two TokenKinds are equal ignoring the embedded values.
    pub fn eq_ignore_value(&self, other: &TokenKind) -> bool {
        use TokenKind::*;
        match (self, other) {
            (Open, Open) => true,
            (Read, Read) => true,
            (Write, Write) => true,
            (Append, Append) => true,
            (Close, Close) => true,
            (Show, Show) => true,
            (Exit, Exit) => true,
            (As, As) => true,
            (Truncate, Truncate) => true,
            (Search, Search) => true,
            (Replace, Replace) => true,
            (LineCount, LineCount) => true,
            (Copy, Copy) => true,
            (Move, Move) => true,
            (Remove, Remove) => true,
            (Rename, Rename) => true,
            (ListDir, ListDir) => true,
            (DumpEnv, DumpEnv) => true,
            (Help, Help) => true,
            (Identifier(_), Identifier(_)) => true,
            (String(_), String(_)) => true,
            (EndOfStatement, EndOfStatement) => true,
            _ => false,
        }
    }

    /// Clone a token kind, taking values from `other` if needed (for identifiers or strings).
    pub fn clone_with_value_from(&self, other: &TokenKind) -> TokenKind {
        match self {
            TokenKind::Identifier(_) => {
                if let TokenKind::Identifier(s) = other {
                    TokenKind::Identifier(s.clone())
                } else {
                    self.clone()
                }
            }
            TokenKind::String(_) => {
                if let TokenKind::String(s) = other {
                    TokenKind::String(s.clone())
                } else {
                    self.clone()
                }
            }
            _ => self.clone(),
        }
    }
}
