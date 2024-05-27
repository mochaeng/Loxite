use std::{
    env, fs,
    io::{self, Write},
    process::exit,
};

use lexer::Lexer;

mod error;
mod lexer;
mod token;

fn run(source: &String) {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens();

    tokens.into_iter().for_each(|token| println!("{:?}", token));
}

fn run_file(path: &String) {
    let source = fs::read_to_string(path).expect("Could not read the file");
    run(&source);

    // if self.had_error {
    //     exit(65);
    // }
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
        // had_error = false;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    if args.len() >= 3 || args.len() == 2 {
        println!("Usage: loxite [script]");
        exit(64);
    }

    if args.len() == 3 {
        run_file(&args[2]);
    } else {
        run_prompt();
    }
}
