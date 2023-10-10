use error::*;
use std::env::args;
use std::io::{self, stdout, BufRead, Write};
mod error;
mod scanner;
mod token;
mod token_type;
use scanner::Scanner;
fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let args = args().collect::<Vec<String>>();
    match args.len() {
        n if (n > 2) => {
            println!("Usage: lox [script]");
            std::process::exit(64);
        }
        n if (n == 2) => run_file(&args[1]).expect("Could not run file"),
        _ => run_prompt(),
    }
    Ok(())
}

fn run_file(filename: &String) -> io::Result<()> {
    let buf = std::fs::read_to_string(filename)?;
    // println!("{:?}", buf);
    if run(buf).is_err() {
        std::process::exit(65);
    }
    Ok(())
}

fn run_prompt() {
    print!("> ");
    let stdin = io::stdin();
    let _ = stdout().flush();
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
            let _ = run(line);
        } else {
            break;
        }
    }
}

fn run(source: String) -> Result<(), LoxError> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;
    for token in tokens {
        println!("{:?}", token);
    }
    Ok(())
}
