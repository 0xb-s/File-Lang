/*!
 * ast.rs
 *
 * Defines the Abstract Syntax Tree (AST) node structures for the language.
 * Each type of statement is represented by a struct. The top-level AST is just a list of statements.
 */

pub struct AST {
    /// A list of statements to be executed in order.
    pub statements: Vec<Statement>,
}

/// A statement in the language.
pub enum Statement {
    Open(OpenStmt),
    Read(ReadStmt),
    Write(WriteStmt),
    Append(AppendStmt),
    Show(ShowStmt),
    Close(CloseStmt),
    Truncate(TruncateStmt),
    Search(SearchStmt),
    Replace(ReplaceStmt),
    LineCount(LineCountStmt),
    Copy(CopyStmt),
    Move(MoveStmt),
    Remove(RemoveStmt),
    Rename(RenameStmt),
    ListDir(ListDirStmt),
    DumpEnv(DumpEnvStmt),
    Help(HelpStmt),
    Exit(ExitStmt),
}

/// The `open` statement node.
pub struct OpenStmt {
    pub filename: String,
    pub var_name: String,
}

/// The `read` statement node.
pub struct ReadStmt {
    pub var_name: String,
}

/// The `write` statement node.
pub struct WriteStmt {
    pub var_name: String,
    pub text: String,
}

/// The `append` statement node.
pub struct AppendStmt {
    pub var_name: String,
    pub text: String,
}

/// The `show` statement node.
pub struct ShowStmt {
    pub var_name: String,
}

/// The `close` statement node.
pub struct CloseStmt {
    pub var_name: String,
}

/// The `truncate` statement node.
pub struct TruncateStmt {
    pub var_name: String,
}

/// The `search` statement node: search var "pattern"
pub struct SearchStmt {
    pub var_name: String,
    pub pattern: String,
}

/// The `replace` statement node: replace var "pattern" "replacement"
pub struct ReplaceStmt {
    pub var_name: String,
    pub pattern: String,
    pub replacement: String,
}

/// The `linecount` statement node.
pub struct LineCountStmt {
    pub var_name: String,
}

/// The `copy` statement node: copy "source" "destination"
pub struct CopyStmt {
    pub source: String,
    pub destination: String,
}

/// The `move` statement node: move "source" "destination"
pub struct MoveStmt {
    pub source: String,
    pub destination: String,
}

/// The `remove` statement node: remove "filename"
pub struct RemoveStmt {
    pub filename: String,
}

/// The `rename` statement node: rename var "newfilename"
pub struct RenameStmt {
    pub var_name: String,
    pub new_filename: String,
}

/// The `listdir` statement node.
pub struct ListDirStmt {
    pub path: String,
}

/// The `dumpenv` statement node.
pub struct DumpEnvStmt;

/// The `help` statement node.
pub struct HelpStmt;

/// The `exit` statement node.
pub struct ExitStmt {}
