use std::{
    env, fs,
    io::{self, Write},
    process::exit,
};

use lexer::Lexer;

mod ast_printer;
mod error;
mod expr;
mod lexer;
mod parser;
mod token;

fn run(source: &String) -> bool {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens();

    tokens.into_iter().for_each(|token| println!("{:?}", token));
    lexer.had_error
}

fn run_file(path: &String) {
    let source = fs::read_to_string(path).expect("Could not read the file");
    let had_error = run(&source);

    if had_error {
        exit(65);
    }
}

fn run_prompt() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        let bytes_read = io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        if bytes_read == 0 {
            break;
        }

        run(&line);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    if args.len() > 2 {
        println!("Usage: loxite [script]");
        exit(64);
    }

    if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}
