mod lexer;
mod tokens;

use lexer::Lexer;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];
    let code = fs::read_to_string(filename).expect("Failed to read input file");

    let mut my_lexer = Lexer::new(&code);

    loop {
        match my_lexer.next_token() {
            Ok(Some(token)) => {
                println!("{:?}", token);
            }
            Ok(None) => {
                std::process::exit(0);
            }
            Err(e) => {
                eprintln!("Lexer error: {}", e);
                std::process::exit(1);
            }
        }
    }
}
