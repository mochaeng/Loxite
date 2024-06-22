use std::{
    env, fs,
    io::{self, Write},
    process::exit,
};

use loxite::{ast_printer::AstPrinter, interpreter::Interpreter, lexer::Lexer, parser::Parser};

struct Loxite {
    interpreter: Interpreter,
}

impl Loxite {
    pub fn new() -> Self {
        Self {
            interpreter: Interpreter {},
        }
    }

    fn run(&self, source: &String) -> Option<i32> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.scan_tokens();
        if lexer.had_error {
            return Some(65);
        }

        let mut parser = Parser::new(tokens);
        let expression = parser.parser();
        if let None = expression {
            return Some(65);
        }

        let expression = expression.unwrap();
        let result_str = AstPrinter.get_expr_as_str(&expression);
        println!("AST: {}", result_str);

        self.interpreter.interpreter(&expression);

        None
    }

    fn run_file(&self, path: &String) {
        let source = fs::read_to_string(path).expect("Could not read the file");
        if let Some(error_code) = self.run(&source) {
            exit(error_code);
        }
    }

    fn run_prompt(&self) {
        loop {
            print!(">> ");
            io::stdout().flush().unwrap();

            let mut line = String::new();
            let bytes_read = io::stdin()
                .read_line(&mut line)
                .expect("Failed to read line");

            if bytes_read == 0 {
                break;
            }

            self.run(&line);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    if args.len() > 2 {
        println!("Usage: loxite [script]");
        exit(64);
    }

    let loxite = Loxite::new();

    if args.len() == 2 {
        loxite.run_file(&args[1]);
    } else {
        loxite.run_prompt();
    }
}
