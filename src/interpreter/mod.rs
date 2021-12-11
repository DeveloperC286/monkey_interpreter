use std::io::{stdin, stdout, Write};

const NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");
const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

use crate::lexical_analysis::LexicalAnalysis;
use crate::syntax_analysis::SyntaxAnalysis;

pub(crate) fn repl() {
    println!(
        "{} {}",
        NAME.unwrap_or("unknown"),
        VERSION.unwrap_or("unknown")
    );
    let mut evaluator = crate::evaluator::Evaluator::new();

    loop {
        match LexicalAnalysis::from(&read()) {
            Ok(tokens) => match SyntaxAnalysis::from(tokens) {
                Ok(abstract_syntax_tree) => match evaluator.evaluate(abstract_syntax_tree) {
                    Ok(object) => println!("{:?}", object),

                    Err(error) => {
                        error!("{}", error);
                    }
                },
                Err(error) => {
                    error!("{}", error);
                }
            },
            Err(error) => {
                error!("{}", error);
            }
        }
    }
}

fn read() -> String {
    let mut buffer = String::new();

    print!(" >>> ");
    let _ = stdout().flush();

    match stdin().read_line(&mut buffer) {
        Ok(_) => {}
        Err(_) => error!("Unable to read user input from standard input."),
    }

    buffer
}
