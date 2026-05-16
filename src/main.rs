mod app_error;
mod expression;
mod interpreter;
mod keywords;
mod lexer;
mod parser;
mod types;

use interpreter::Interpreter;
use std::{env, fs, path::Path};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("Usage: lolcoders <file1.lol> [file2.lol ...]");
        std::process::exit(1);
    }

    for filename in args {
        // Check extension
        if Path::new(&filename)
            .extension()
            .and_then(|ext| ext.to_str())
            != Some("lol")
        {
            eprintln!("Error: '{}' is not a .lol file", filename);
            std::process::exit(1);
        }

        let source = match fs::read_to_string(&filename) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to read '{}': {}", filename, e);
                std::process::exit(1);
            }
        };

        let mut interpreter = Interpreter::new();

        if let Err(e) = interpreter.execute_source(source) {
            eprintln!("Error in '{}': {}", filename, e);
            std::process::exit(1);
        }
    }
}
