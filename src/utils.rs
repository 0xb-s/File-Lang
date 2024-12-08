/*!
 * utils.rs
 *
 * Utility functions for the language. Includes:
 * - Character classification for the lexer.
 * - Helpers for file and directory operations.
 * - Helpers for regex-based searching.
 */

use regex::Regex;
use std::fs;
use std::fs::OpenOptions;
use std::io;

/// Check if a character can be part of an identifier.
pub fn is_identifier_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '-' || c == '.'
}

/// List the files in a directory. Returns a vector of filenames.
pub fn list_directory(path: &str) -> io::Result<Vec<String>> {
    let mut results = Vec::new();
    let entries = fs::read_dir(path)?;
    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy().to_string();
        results.push(file_name_str);
    }
    Ok(results)
}

/// Move (rename) a file from src to dst.
pub fn move_file(src: &str, dst: &str) -> io::Result<()> {
    fs::rename(src, dst)
}

/// Copy a file from src to dst.
pub fn copy_file(src: &str, dst: &str) -> io::Result<u64> {
    fs::copy(src, dst)
}

/// Remove a file.
pub fn remove_file(path: &str) -> io::Result<()> {
    fs::remove_file(path)
}

/// Write a string to a file (overwriting).
pub fn write_to_file(filename: &str, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(filename)?;
    use std::io::Write;
    file.write_all(content.as_bytes())?;
    Ok(())
}

/// Append a string to a file.
pub fn append_to_file(filename: &str, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(filename)?;
    use std::io::Write;
    file.write_all(content.as_bytes())?;
    Ok(())
}

/// Read the entire content of a file.
pub fn read_file_content(filename: &str) -> io::Result<String> {
    fs::read_to_string(filename)
}

/// Search for a regex pattern in a text. Returns vector of (line_number, line) for matches.
pub fn search_in_text(text: &str, pattern: &str) -> Result<Vec<(usize, String)>, String> {
    let re = Regex::new(pattern).map_err(|e| e.to_string())?;
    let mut results = Vec::new();
    for (i, line) in text.lines().enumerate() {
        if re.is_match(line) {
            results.push((i + 1, line.to_string()));
        }
    }
    Ok(results)
}

/// Replace a regex pattern in a text with a replacement. Returns the replaced string.
pub fn replace_in_text(text: &str, pattern: &str, replacement: &str) -> Result<String, String> {
    let re = Regex::new(pattern).map_err(|e| e.to_string())?;
    Ok(re.replace_all(text, replacement).to_string())
}
