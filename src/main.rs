mod app_error;
mod interpreter;
mod lexer;
mod parser;
mod types;

use interpreter::Interpreter;
use std::fs;

fn main() {
    let source = fs::read_to_string("program.lol").expect("Failed to read file");

    let mut interpreter = Interpreter::new();
    match interpreter.execute_source(source) {
        Ok(_) => (),
        Err(e) => println!("{e}"),
    }
}
