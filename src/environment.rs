/*!
 * environment.rs
 *
 * The Environment manages the runtime state:
 * - A mapping from variable names to FileEntry (filename, content, and open state)
 *
 * The environment stores the in-memory content of opened files. Operations like `read`, `write`,
 * `append`, `search`, `replace`, `truncate`, `linecount`, `rename`, and `close` all act on
 * these environment entries.
 *
 * If a file operation requires disk access, the environment methods handle it.
 */

use crate::errors::RuntimeError;
use crate::utils::{
    append_to_file, read_file_content, replace_in_text, search_in_text, write_to_file,
};
use std::collections::HashMap;

pub struct Environment {
    /// Map of variable names to file entries.
    pub files: HashMap<String, FileEntry>,
}

/// A file entry holds the state of an opened file.
pub struct FileEntry {
    /// The filename on disk.
    pub filename: String,
    /// The in-memory content of the file.
    pub content: String,
    /// Whether the file is currently open.
    pub is_open: bool,
}

impl Environment {
    /// Create a new empty environment.
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
        }
    }

    /// Open a file and assign it to a variable.
    /// If already open, error unless it was closed previously.
    pub fn open_file(&mut self, var_name: String, filename: String) -> Result<(), RuntimeError> {
        if self.files.contains_key(&var_name) {
            let entry = self.files.get_mut(&var_name).unwrap();
            if entry.is_open {
                return Err(RuntimeError::new(format!(
                    "Variable '{}' already has an open file.",
                    var_name
                )));
            } else {
                // If the file is closed, we can reuse and update filename
                entry.filename = filename;
                entry.content.clear();
                entry.is_open = true;
            }
        } else {
            self.files.insert(
                var_name,
                FileEntry {
                    filename,
                    content: String::new(),
                    is_open: true,
                },
            );
        }
        Ok(())
    }

    /// Read file content from disk into the environment.
    pub fn read_file_content(&mut self, var_name: &str) -> Result<(), RuntimeError> {
        let entry = self.get_entry_mut(var_name)?;
        let filename = &entry.filename;
        let buffer = read_file_content(filename)
            .map_err(|e| RuntimeError::new(format!("Failed to read file '{}': {}", filename, e)))?;
        entry.content = buffer;
        Ok(())
    }

    /// Write new content to the file (overwrite) and memory.
    pub fn write_file_content(&mut self, var_name: &str, text: &str) -> Result<(), RuntimeError> {
        let entry = self.get_entry_mut(var_name)?;
        let filename = &entry.filename;
        write_to_file(filename, text).map_err(|e| {
            RuntimeError::new(format!("Failed to write to file '{}': {}", filename, e))
        })?;
        entry.content = text.to_string();
        Ok(())
    }

    /// Append text to the file content in memory and on disk.
    pub fn append_file_content(&mut self, var_name: &str, text: &str) -> Result<(), RuntimeError> {
        let entry = self.get_entry_mut(var_name)?;
        let filename = &entry.filename;
        append_to_file(filename, text).map_err(|e| {
            RuntimeError::new(format!("Failed to append to file '{}': {}", filename, e))
        })?;
        entry.content.push_str(text);
        Ok(())
    }

    /// Get the content of a file in memory.
    pub fn get_file_content(&self, var_name: &str) -> Result<String, RuntimeError> {
        let entry = self.get_entry(var_name)?;
        Ok(entry.content.clone())
    }

    /// Close a file.
    pub fn close_file(&mut self, var_name: &str) -> Result<(), RuntimeError> {
        let entry = self.get_entry_mut(var_name)?;
        if !entry.is_open {
            return Err(RuntimeError::new(format!(
                "Variable '{}' file is not open.",
                var_name
            )));
        }
        entry.is_open = false;
        Ok(())
    }

    /// Truncate a file: clear its content both in memory and on disk.
    pub fn truncate_file(&mut self, var_name: &str) -> Result<(), RuntimeError> {
        let entry = self.get_entry_mut(var_name)?;
        let filename = &entry.filename;
        write_to_file(filename, "").map_err(|e| {
            RuntimeError::new(format!("Failed to truncate file '{}': {}", filename, e))
        })?;
        entry.content.clear();
        Ok(())
    }

    /// Search for a regex pattern in the file content and return matches.
    pub fn search_file(
        &self,
        var_name: &str,
        pattern: &str,
    ) -> Result<Vec<(usize, String)>, RuntimeError> {
        let entry = self.get_entry(var_name)?;
        let content = &entry.content;
        let matches = search_in_text(content, pattern)
            .map_err(|e| RuntimeError::new(format!("Invalid regex '{}': {}", pattern, e)))?;
        Ok(matches)
    }

    /// Replace occurrences of a pattern with replacement in the file.
    pub fn replace_file(
        &mut self,
        var_name: &str,
        pattern: &str,
        replacement: &str,
    ) -> Result<(), RuntimeError> {
        let entry = self.get_entry_mut(var_name)?;
        let new_content = replace_in_text(&entry.content, pattern, replacement)
            .map_err(|e| RuntimeError::new(format!("Invalid regex '{}': {}", pattern, e)))?;
        let filename = &entry.filename;
        write_to_file(filename, &new_content).map_err(|e| {
            RuntimeError::new(format!(
                "Failed to write replaced content to file '{}': {}",
                filename, e
            ))
        })?;
        entry.content = new_content;
        Ok(())
    }

    /// Count lines in a file's content.
    pub fn line_count(&self, var_name: &str) -> Result<usize, RuntimeError> {
        let entry = self.get_entry(var_name)?;
        Ok(entry.content.lines().count())
    }

    /// Rename the file associated with a variable and update the environment.
    pub fn rename_file(&mut self, var_name: &str, new_filename: &str) -> Result<(), RuntimeError> {
        let entry = self.get_entry_mut(var_name)?;
        let old_filename = &entry.filename;
        std::fs::rename(old_filename, new_filename).map_err(|e| {
            RuntimeError::new(format!(
                "Failed to rename file '{}' to '{}': {}",
                old_filename, new_filename, e
            ))
        })?;
        entry.filename = new_filename.to_string();
        Ok(())
    }

    /// Get read-only reference to a file entry.
    fn get_entry(&self, var_name: &str) -> Result<&FileEntry, RuntimeError> {
        let entry = self
            .files
            .get(var_name)
            .ok_or_else(|| RuntimeError::new(format!("No such variable '{}'", var_name)))?;
        if !entry.is_open {
            return Err(RuntimeError::new(format!(
                "Variable '{}' file is not open.",
                var_name
            )));
        }
        Ok(entry)
    }

    /// Get mutable reference to a file entry.
    fn get_entry_mut(&mut self, var_name: &str) -> Result<&mut FileEntry, RuntimeError> {
        let entry = self
            .files
            .get_mut(var_name)
            .ok_or_else(|| RuntimeError::new(format!("No such variable '{}'", var_name)))?;
        if !entry.is_open {
            return Err(RuntimeError::new(format!(
                "Variable '{}' file is not open.",
                var_name
            )));
        }
        Ok(entry)
    }

    /// Dump the environment: list all variables and their files, open state.
    pub fn dump(&self) {
        println!("Environment Variables:");
        if self.files.is_empty() {
            println!("  (none)");
        }
        for (var, entry) in &self.files {
            let state = if entry.is_open { "open" } else { "closed" };
            println!("  {} -> {} [{}]", var, entry.filename, state);
        }
    }
}
