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
        let tokens = LexicalAnalysis::from(&read());
        let abstract_syntax_tree = SyntaxAnalysis::from(tokens);

        if !abstract_syntax_tree.syntax_parsing_errors.is_empty() {
            for error in abstract_syntax_tree
                .syntax_parsing_errors
                .iter()
                .enumerate()
            {
                error!("{:?}", error);
            }
        } else {
            let object = evaluator.evaluate(abstract_syntax_tree);
            println!("{:?}", object);
        }
    }
}

fn read() -> String {
    let mut buffer = String::new();

    print!(" >>> ");
    let _ = stdout().flush();

    match stdin().read_line(&mut buffer) {
        Ok(_) => {}
        Err(error) => error!("Error reading user input: {}", error),
    }

    buffer
}
