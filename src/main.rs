/*!
 * main.rs
 *
 * Entry point for the extended file manipulation language interpreter.
 * This version supports numerous commands for manipulating files, searching,
 * replacing, listing directories, copying/moving files, removing files, dumping
 * environment variables, and more.
 *
 * Usage:
 *   cargo run -- script.txt
 * or
 *   cargo run
 *   (then type commands directly)
 *
 * Example:
 *   open "example.txt" as f
 *   write f "Hello World!\nThis is a test."
 *   read f
 *   show f
 *   search f "World"
 *   replace f "test" "demo"
 *   show f
 *   linecount f
 *   truncate f
 *   show f
 *   close f
 *   exit
 */

use file_lang::{interpreter::Interpreter, lexer::Lexer, parser::Parser};
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut source = String::new();

    if args.len() > 1 {
        let filename = &args[1];
        let file = File::open(filename).expect("Unable to open input script file.");
        let mut reader = BufReader::new(file);
        reader
            .read_to_string(&mut source)
            .expect("Unable to read from input file.");
    } else {
        let mut stdin = std::io::stdin();
        let mut buffer = Vec::new();
        let _ = std::io::Read::read_to_end(&mut stdin, &mut buffer).expect("Failed to read stdin.");
        source = String::from_utf8(buffer).expect("Invalid UTF-8 in stdin input.");
    }

    let mut lexer = Lexer::new(&source);
    let tokens = match lexer.lex() {
        Ok(toks) => toks,
        Err(e) => {
            eprintln!("Lexing error: {}", e);
            std::process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("Parsing error: {}", e);
            std::process::exit(1);
        }
    };

    let mut interpreter = Interpreter::new();
    if let Err(e) = interpreter.run(&ast) {
        eprintln!("Runtime error: {}", e);
        std::process::exit(1);
    }
}
