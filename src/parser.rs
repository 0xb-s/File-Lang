/*!
 * parser.rs
 *
 * The Parser converts a sequence of tokens into an Abstract Syntax Tree (AST).
 *
 * This parser recognizes the extended grammar defined above.
 * It consumes tokens produced by the lexer and builds the AST.
 *
 * If parsing fails, returns a ParseError.
 */

use crate::ast::*;
use crate::errors::ParseError;
use crate::tokens::{Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    length: usize,
}

impl Parser {
    /// Create a new parser with a list of tokens.
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pos: 0,
            length: 0,
        }
    }

    /// Parse the entire token stream into an AST.
    pub fn parse(&mut self) -> Result<AST, ParseError> {
        self.length = self.tokens.len();
        let mut statements = Vec::new();

        while !self.is_at_end() {
            if self.check_end_of_statement() {
                self.consume_end_of_statement()?;
                continue;
            }

            let stmt = self.parse_statement()?;
            statements.push(stmt);
            if !self.is_at_end() && !self.check_end_of_statement() {
                return Err(ParseError::new(format!(
                    "Expected end of statement at position {} but found {:?}",
                    self.current_position(),
                    self.peek_token().kind
                )));
            }
            self.consume_end_of_statement().ok();
        }

        Ok(AST { statements })
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        if self.match_token(&[TokenKind::Open]) {
            let filename = self.consume_expect_string("Expected filename string after 'open'")?;
            self.consume_expect_token(
                TokenKind::As,
                "Expected 'as' after filename in open statement",
            )?;
            let var = self.consume_expect_identifier("Expected variable name after 'as'")?;
            return Ok(Statement::Open(OpenStmt {
                filename,
                var_name: var,
            }));
        }

        if self.match_token(&[TokenKind::Read]) {
            let var = self.consume_expect_identifier("Expected variable name after 'read'")?;
            return Ok(Statement::Read(ReadStmt { var_name: var }));
        }

        if self.match_token(&[TokenKind::Write]) {
            let var = self.consume_expect_identifier("Expected variable name after 'write'")?;
            let text = self.consume_expect_string("Expected string after variable in 'write'")?;
            return Ok(Statement::Write(WriteStmt {
                var_name: var,
                text,
            }));
        }

        if self.match_token(&[TokenKind::Append]) {
            let var = self.consume_expect_identifier("Expected variable name after 'append'")?;
            let text = self.consume_expect_string("Expected string after variable in 'append'")?;
            return Ok(Statement::Append(AppendStmt {
                var_name: var,
                text,
            }));
        }

        if self.match_token(&[TokenKind::Show]) {
            let var = self.consume_expect_identifier("Expected variable name after 'show'")?;
            return Ok(Statement::Show(ShowStmt { var_name: var }));
        }

        if self.match_token(&[TokenKind::Close]) {
            let var = self.consume_expect_identifier("Expected variable name after 'close'")?;
            return Ok(Statement::Close(CloseStmt { var_name: var }));
        }

        if self.match_token(&[TokenKind::Truncate]) {
            let var = self.consume_expect_identifier("Expected variable name after 'truncate'")?;
            return Ok(Statement::Truncate(TruncateStmt { var_name: var }));
        }

        if self.match_token(&[TokenKind::Search]) {
            let var = self.consume_expect_identifier("Expected variable name after 'search'")?;
            let pattern =
                self.consume_expect_string("Expected pattern string after variable in 'search'")?;
            return Ok(Statement::Search(SearchStmt {
                var_name: var,
                pattern,
            }));
        }

        if self.match_token(&[TokenKind::Replace]) {
            let var = self.consume_expect_identifier("Expected variable name after 'replace'")?;
            let pattern =
                self.consume_expect_string("Expected pattern string after variable in 'replace'")?;
            let replacement = self
                .consume_expect_string("Expected replacement string after pattern in 'replace'")?;
            return Ok(Statement::Replace(ReplaceStmt {
                var_name: var,
                pattern,
                replacement,
            }));
        }

        if self.match_token(&[TokenKind::LineCount]) {
            let var = self.consume_expect_identifier("Expected variable name after 'linecount'")?;
            return Ok(Statement::LineCount(LineCountStmt { var_name: var }));
        }

        if self.match_token(&[TokenKind::Copy]) {
            let src = self.consume_expect_string("Expected source filename after 'copy'")?;
            let dst =
                self.consume_expect_string("Expected destination filename after source in 'copy'")?;
            return Ok(Statement::Copy(CopyStmt {
                source: src,
                destination: dst,
            }));
        }

        if self.match_token(&[TokenKind::Move]) {
            let src = self.consume_expect_string("Expected source filename after 'move'")?;
            let dst =
                self.consume_expect_string("Expected destination filename after source in 'move'")?;
            return Ok(Statement::Move(MoveStmt {
                source: src,
                destination: dst,
            }));
        }

        if self.match_token(&[TokenKind::Remove]) {
            let fname = self.consume_expect_string("Expected filename after 'remove'")?;
            return Ok(Statement::Remove(RemoveStmt { filename: fname }));
        }

        if self.match_token(&[TokenKind::Rename]) {
            let var = self.consume_expect_identifier("Expected variable name after 'rename'")?;
            let new_fname =
                self.consume_expect_string("Expected new filename after variable in 'rename'")?;
            return Ok(Statement::Rename(RenameStmt {
                var_name: var,
                new_filename: new_fname,
            }));
        }

        if self.match_token(&[TokenKind::ListDir]) {
            let path = self.consume_expect_string("Expected directory path after 'listdir'")?;
            return Ok(Statement::ListDir(ListDirStmt { path }));
        }

        if self.match_token(&[TokenKind::DumpEnv]) {
            return Ok(Statement::DumpEnv(DumpEnvStmt {}));
        }

        if self.match_token(&[TokenKind::Help]) {
            return Ok(Statement::Help(HelpStmt {}));
        }

        if self.match_token(&[TokenKind::Exit]) {
            return Ok(Statement::Exit(ExitStmt {}));
        }

        Err(ParseError::new(format!(
            "Unexpected token {:?} at position {}",
            self.peek_token().kind,
            self.current_position()
        )))
    }

    fn is_at_end(&self) -> bool {
        self.pos >= self.length
    }

    fn peek_token(&self) -> &Token {
        if self.is_at_end() {
            &self.tokens[self.length - 1]
        } else {
            &self.tokens[self.pos]
        }
    }

    fn current_position(&self) -> usize {
        self.pos
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.pos += 1;
        }
        &self.tokens[self.pos - 1]
    }

    fn match_token(&mut self, kinds: &[TokenKind]) -> bool {
        if self.is_at_end() {
            return false;
        }
        for kind in kinds {
            if self.peek_token().kind.eq_ignore_value(kind) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check_end_of_statement(&self) -> bool {
        if self.is_at_end() {
            return true;
        }
        let tk = &self.peek_token().kind;
        match tk {
            TokenKind::EndOfStatement => true,
            _ => false,
        }
    }

    fn consume_end_of_statement(&mut self) -> Result<(), ParseError> {
        if self.check_end_of_statement() {
            self.advance();
            Ok(())
        } else if self.is_at_end() {
            // End of file is also considered end of statement
            Ok(())
        } else {
            Err(ParseError::new("Expected end of statement".to_string()))
        }
    }

    fn consume_expect_string(&mut self, err_msg: &str) -> Result<String, ParseError> {
        if self.is_at_end() {
            return Err(ParseError::new(err_msg.to_string()));
        }
        let tk = self.advance();
        match &tk.kind {
            TokenKind::String(s) => Ok(s.clone()),
            _ => Err(ParseError::new(format!("{}: got {:?}", err_msg, tk.kind))),
        }
    }

    fn consume_expect_token(
        &mut self,
        expected: TokenKind,
        err_msg: &str,
    ) -> Result<TokenKind, ParseError> {
        if self.is_at_end() {
            return Err(ParseError::new(err_msg.to_string()));
        }
        let tk = self.advance();
        if tk.kind.eq_ignore_value(&expected) {
            Ok(expected.clone_with_value_from(&tk.kind))
        } else {
            Err(ParseError::new(format!("{}: got {:?}", err_msg, tk.kind)))
        }
    }

    fn consume_expect_identifier(&mut self, err_msg: &str) -> Result<String, ParseError> {
        if self.is_at_end() {
            return Err(ParseError::new(err_msg.to_string()));
        }
        let tk = self.advance();
        match &tk.kind {
            TokenKind::Identifier(s) => Ok(s.clone()),
            _ => Err(ParseError::new(format!("{}: got {:?}", err_msg, tk.kind))),
        }
    }
}
