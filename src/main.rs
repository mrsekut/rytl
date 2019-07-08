mod error;
mod interp;
mod lexer;
mod parser;
mod util;
use std::io;

fn prompt(s: &str) -> io::Result<()> {
    use std::io::{stdout, Write};

    let stdout = stdout();
    let mut stdout = stdout.lock();
    stdout.write(s.as_bytes())?;
    stdout.flush()
}

fn run_lexer() {
    use std::io::{self, BufRead, BufReader};
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let stdin = BufReader::new(stdin);
    let mut lines = stdin.lines();

    loop {
        prompt("> ").unwrap();
        if let Some(Ok(line)) = lines.next() {
            let token = match lexer::lexer(&line) {
                Ok(token) => token,
                Err(e) => {
                    error::show_trace(e);
                    continue;
                }
            };
            println!("{:?}", token);
        } else {
            break;
        }
    }
}

fn run_parser() {
    use std::io::{self, BufRead, BufReader};

    let stdin = io::stdin();
    let stdin = stdin.lock();
    let stdin = BufReader::new(stdin);
    let mut lines = stdin.lines();

    loop {
        prompt("> ").unwrap();
        if let Some(Ok(line)) = lines.next() {
            let ast = match line.parse::<parser::Ast>() {
                Ok(ast) => ast,
                Err(e) => {
                    e.show_diagnostic(&line);
                    error::show_trace(e);
                    continue;
                }
            };
            println!("{:?}", ast);
        } else {
            break;
        }
    }
}

fn run_eval() {
    use std::io::{self, BufRead, BufReader};
    let mut interp = interp::Interpreter::new();

    let stdin = io::stdin();
    let stdin = stdin.lock();
    let stdin = BufReader::new(stdin);
    let mut lines = stdin.lines();

    loop {
        prompt("> ").unwrap();
        if let Some(Ok(line)) = lines.next() {
            let ast = match line.parse::<parser::Ast>() {
                Ok(ast) => ast,
                Err(e) => {
                    e.show_diagnostic(&line);
                    error::show_trace(e);
                    continue;
                }
            };
            let n = match interp.eval(&ast) {
                Ok(n) => n,
                Err(e) => {
                    e.show_diagnostic(&line);
                    error::show_trace(e);
                    continue;
                }
            };
            println!("{}", n);
        } else {
            break;
        }
    }
}

fn main() {
    // run_lexer();
    // run_parser();
    run_eval();
}

// use itertools::multipeek;
// use itertools::MultiPeek;

// // impl<T: Iterator> Copy for MultiPeek<T> {}
// // impl<T> Copy for MultiPeek<T> where T: Iterator {}

// fn main() {
//     let it = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
//     let mut it = it.into_iter();
//     for i in 1..5 {
//         multipeek(it).peek().map(|a| println!("{}", a));
//     }
//     println!("{:?}", itertools::multipeek(it).peek());
// }
