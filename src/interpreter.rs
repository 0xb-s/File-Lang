/*!
 * interpreter.rs
 *
 * The Interpreter executes the AST. It uses:
 * - An Environment for file variables.
 *
 * For each statement in the AST, the interpreter performs the corresponding action.
 * Errors at runtime return a RuntimeError.
 */

use crate::ast::*;
use crate::commands::help_text;
use crate::environment::Environment;
use crate::errors::RuntimeError;
use crate::utils::{copy_file, list_directory, move_file, remove_file};

pub struct Interpreter {
    env: Environment,
    stop: bool,
}

impl Interpreter {
    /// Create a new interpreter.
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
            stop: false,
        }
    }

    /// Run the given AST in the interpreter.
    pub fn run(&mut self, ast: &AST) -> Result<(), RuntimeError> {
        for stmt in &ast.statements {
            if self.stop {
                break;
            }
            self.execute_statement(stmt)?;
        }
        Ok(())
    }

    /// Execute a single statement.
    fn execute_statement(&mut self, stmt: &Statement) -> Result<(), RuntimeError> {
        match stmt {
            Statement::Open(s) => self.execute_open(&s.filename, &s.var_name),
            Statement::Read(s) => self.execute_read(&s.var_name),
            Statement::Write(s) => self.execute_write(&s.var_name, &s.text),
            Statement::Append(s) => self.execute_append(&s.var_name, &s.text),
            Statement::Show(s) => self.execute_show(&s.var_name),
            Statement::Close(s) => self.execute_close(&s.var_name),
            Statement::Truncate(s) => self.execute_truncate(&s.var_name),
            Statement::Search(s) => self.execute_search(&s.var_name, &s.pattern),
            Statement::Replace(s) => self.execute_replace(&s.var_name, &s.pattern, &s.replacement),
            Statement::LineCount(s) => self.execute_linecount(&s.var_name),
            Statement::Copy(s) => self.execute_copy(&s.source, &s.destination),
            Statement::Move(s) => self.execute_move(&s.source, &s.destination),
            Statement::Remove(s) => self.execute_remove(&s.filename),
            Statement::Rename(s) => self.execute_rename(&s.var_name, &s.new_filename),
            Statement::ListDir(s) => self.execute_listdir(&s.path),
            Statement::DumpEnv(_) => self.execute_dumpenv(),
            Statement::Help(_) => self.execute_help(),
            Statement::Exit(_) => self.execute_exit(),
        }
    }

    fn execute_open(&mut self, filename: &str, var_name: &str) -> Result<(), RuntimeError> {
        self.env
            .open_file(var_name.to_string(), filename.to_string())
    }

    fn execute_read(&mut self, var_name: &str) -> Result<(), RuntimeError> {
        self.env.read_file_content(var_name)
    }

    fn execute_write(&mut self, var_name: &str, text: &str) -> Result<(), RuntimeError> {
        self.env.write_file_content(var_name, text)
    }

    fn execute_append(&mut self, var_name: &str, text: &str) -> Result<(), RuntimeError> {
        self.env.append_file_content(var_name, text)
    }

    fn execute_show(&mut self, var_name: &str) -> Result<(), RuntimeError> {
        let content = self.env.get_file_content(var_name)?;
        println!("{}", content);
        Ok(())
    }

    fn execute_close(&mut self, var_name: &str) -> Result<(), RuntimeError> {
        self.env.close_file(var_name)
    }

    fn execute_truncate(&mut self, var_name: &str) -> Result<(), RuntimeError> {
        self.env.truncate_file(var_name)
    }

    fn execute_search(&mut self, var_name: &str, pattern: &str) -> Result<(), RuntimeError> {
        let matches = self.env.search_file(var_name, pattern)?;
        if matches.is_empty() {
            println!("No matches found.");
        } else {
            for (line_num, line) in matches {
                println!("{}: {}", line_num, line);
            }
        }
        Ok(())
    }

    fn execute_replace(
        &mut self,
        var_name: &str,
        pattern: &str,
        replacement: &str,
    ) -> Result<(), RuntimeError> {
        self.env.replace_file(var_name, pattern, replacement)
    }

    fn execute_linecount(&mut self, var_name: &str) -> Result<(), RuntimeError> {
        let count = self.env.line_count(var_name)?;
        println!("{} lines", count);
        Ok(())
    }

    fn execute_copy(&mut self, source: &str, destination: &str) -> Result<(), RuntimeError> {
        copy_file(source, destination).map_err(|e| {
            RuntimeError::new(format!(
                "Failed to copy file '{}' to '{}': {}",
                source, destination, e
            ))
        })?;
        Ok(())
    }

    fn execute_move(&mut self, source: &str, destination: &str) -> Result<(), RuntimeError> {
        move_file(source, destination).map_err(|e| {
            RuntimeError::new(format!(
                "Failed to move file '{}' to '{}': {}",
                source, destination, e
            ))
        })?;
        Ok(())
    }

    fn execute_remove(&mut self, filename: &str) -> Result<(), RuntimeError> {
        remove_file(filename).map_err(|e| {
            RuntimeError::new(format!("Failed to remove file '{}': {}", filename, e))
        })?;
        Ok(())
    }

    fn execute_rename(&mut self, var_name: &str, new_filename: &str) -> Result<(), RuntimeError> {
        self.env.rename_file(var_name, new_filename)
    }

    fn execute_listdir(&mut self, path: &str) -> Result<(), RuntimeError> {
        let listing = list_directory(path).map_err(|e| {
            RuntimeError::new(format!("Failed to list directory '{}': {}", path, e))
        })?;
        if listing.is_empty() {
            println!("(empty directory)");
        } else {
            for fname in listing {
                println!("{}", fname);
            }
        }
        Ok(())
    }

    fn execute_dumpenv(&mut self) -> Result<(), RuntimeError> {
        self.env.dump();
        Ok(())
    }

    fn execute_help(&mut self) -> Result<(), RuntimeError> {
        println!("{}", help_text());
        Ok(())
    }

    fn execute_exit(&mut self) -> Result<(), RuntimeError> {
        self.stop = true;
        Ok(())
    }
}
