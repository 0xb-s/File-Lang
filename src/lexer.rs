/*!
 * lexer.rs
 *
 * The Lexer converts raw input into tokens.
 *
 * It supports keywords:
 * open, read, write, append, show, close, exit, as, truncate, search, replace,
 * linecount, copy, move, remove, rename, listdir, dumpenv, help
 *
 * Identifiers: used for variables.
 * Strings: double-quoted strings for filenames, patterns, and text.
 * EndOfStatement: newline or semicolon
 * Comments: lines starting with '#' are ignored until newline.
 */

use crate::errors::LexError;
use crate::tokens::{Token, TokenKind};
use crate::utils::is_identifier_char;

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
    length: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            pos: 0,
            length: input.len(),
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            self.skip_whitespace();
            if self.is_at_end() {
                break;
            }

            let c = self.peek_char();
            if c == '"' {
                let start = self.pos;
                let string_val = self.lex_string()?;
                tokens.push(Token::new(TokenKind::String(string_val), start));
                continue;
            }

            if c.is_alphabetic() {
                let start = self.pos;
                let ident = self.lex_identifier();
                let kind = self.ident_to_keyword_or_identifier(&ident);
                tokens.push(Token::new(kind, start));
                continue;
            }

            if c == '\n' {
                let start = self.pos;
                self.pos += 1;
                tokens.push(Token::new(TokenKind::EndOfStatement, start));
                continue;
            }

            if c == ';' {
                let start = self.pos;
                self.pos += 1;
                tokens.push(Token::new(TokenKind::EndOfStatement, start));
                continue;
            }

            if c == '#' {
                self.lex_comment();
                continue;
            }

            return Err(LexError::new(format!(
                "Unexpected character '{}' at position {}",
                c, self.pos
            )));
        }

     
        tokens.push(Token::new(TokenKind::EndOfStatement, self.pos));
        Ok(tokens)
    }

    fn is_at_end(&self) -> bool {
        self.pos >= self.length
    }

    fn peek_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    fn advance(&mut self) -> char {
        let c = self.input[self.pos..].chars().next().unwrap();
        let char_len = c.len_utf8();
        self.pos += char_len;
        c
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            let c = self.peek_char();
            if c.is_whitespace() && c != '\n' {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn lex_string(&mut self) -> Result<String, LexError> {
        self.advance(); // consume "
        let start = self.pos;
        let mut result = String::new();
        while !self.is_at_end() {
            let c = self.peek_char();
            if c == '"' {
                self.advance(); 
                return Ok(result);
            } else {
                result.push(c);
                self.advance();
            }
        }
        Err(LexError::new(format!(
            "Unterminated string starting at position {}",
            start
        )))
    }

    fn lex_identifier(&mut self) -> String {
        let mut result = String::new();
        while !self.is_at_end() && is_identifier_char(self.peek_char()) {
            result.push(self.advance());
        }
        result
    }

    fn lex_comment(&mut self) {
        while !self.is_at_end() {
            let c = self.peek_char();
            if c == '\n' {
                break;
            } else {
                self.advance();
            }
        }
    }

    fn ident_to_keyword_or_identifier(&self, ident: &str) -> TokenKind {
        match ident.to_lowercase().as_str() {
            "open" => TokenKind::Open,
            "read" => TokenKind::Read,
            "write" => TokenKind::Write,
            "append" => TokenKind::Append,
            "show" => TokenKind::Show,
            "close" => TokenKind::Close,
            "exit" => TokenKind::Exit,
            "as" => TokenKind::As,
            "truncate" => TokenKind::Truncate,
            "search" => TokenKind::Search,
            "replace" => TokenKind::Replace,
            "linecount" => TokenKind::LineCount,
            "copy" => TokenKind::Copy,
            "move" => TokenKind::Move,
            "remove" => TokenKind::Remove,
            "rename" => TokenKind::Rename,
            "listdir" => TokenKind::ListDir,
            "dumpenv" => TokenKind::DumpEnv,
            "help" => TokenKind::Help,
            _ => TokenKind::Identifier(ident.to_string()),
        }
    }
}
