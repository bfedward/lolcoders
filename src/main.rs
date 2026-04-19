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
    let args = env::args().skip(1);

    if args.len() == 0 {
        eprintln!("Usage: lcr <file1.lol> [file2.lol ...]");
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
            continue;
        }

        let source = match fs::read_to_string(&filename) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to read '{}': {}", filename, e);
                continue;
            }
        };

        let mut interpreter = Interpreter::new();

        if let Err(e) = interpreter.execute_source(source) {
            eprintln!("Error in '{}': {}", filename, e);
        }
    }
}
