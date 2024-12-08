/// Return a help string listing all commands and their usage.
pub fn help_text() -> String {
    let text = r#"
Available commands:

Basic File Operations:
  open "filename" as var      - Open a file and assign it to a variable
  read var                    - Read the file content from disk into memory
  write var "text"            - Overwrite the file with the given text
  append var "text"           - Append text to the end of the file
  show var                    - Print the in-memory content of the file
  close var                   - Close the file associated with the variable
  truncate var                - Clear the file content (both in memory and on disk)

Advanced File Operations:
  search var "pattern"        - Search for regex pattern in the file content
  replace var "pattern" "replacement"
                             - Replace all occurrences of the pattern with the replacement
  linecount var               - Show the number of lines in the file
  rename var "newfilename"    - Rename the file associated with var

File System Operations:
  copy "source" "destination" - Copy a file on disk
  move "source" "destination" - Move/rename a file on disk
  remove "filename"           - Remove a file from disk

Directory and Environment:
  listdir "path"              - List files in a directory
  dumpenv                     - Show all variables, their files, and open/closed state

Miscellaneous:
  help                        - Show this help message
  exit                        - Exit the interpreter

End of Statement:
  Statements can end with a newline or a semicolon.

Note:
  Patterns are regular expressions (using Rust's 'regex' crate syntax).
  Filenames and text must be in double quotes.
"#;
    text.to_string()
}
