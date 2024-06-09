use std::{
    env, fs,
    io::{self, Write},
    process::exit,
};

use loxite::{ast_printer::AstPrinter, lexer::Lexer, parser::Parser};

fn run(source: &String) {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens();
    if lexer.had_error {
        return;
    }

    let mut parser = Parser::new(tokens);
    let expression = parser.parser();
    if let None = expression {
        return;
    }

    let expression = expression.unwrap();
    let result_str = AstPrinter.get_expr_as_str(expression);
    println!("{}", result_str);
}

fn run_file(path: &String) {
    let source = fs::read_to_string(path).expect("Could not read the file");
    run(&source);
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
