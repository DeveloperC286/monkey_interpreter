use super::lexical_analysis::LexicalAnalysis;
use super::syntax_analysis::SyntaxAnalysis;

use std::io::{stdin, stdout, Write};

const NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");
const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

pub fn repl() {
    println!(
        "{} {}",
        NAME.unwrap_or("unknown"),
        VERSION.unwrap_or("unknown")
    );
    loop {
        let tokens = LexicalAnalysis::get_tokens(read());
        let abstract_syntax_tree = SyntaxAnalysis::get_abstract_syntax_tree(tokens);

        if abstract_syntax_tree.syntax_parsing_errors.len() > 0 {
            for error in abstract_syntax_tree
                .syntax_parsing_errors
                .iter()
                .enumerate()
            {
                error!("{:?}", error);
            }
        } else {
            //TODO Handle the abstract_syntax_tree.
        }
    }
}

fn read() -> String {
    let mut buffer = String::new();

    print!(" >>> ");
    let _ = stdout().flush();

    match stdin().read_line(&mut buffer) {
        Ok(_n) => {}
        Err(error) => error!("Error reading user input: {}", error),
    }

    return buffer;
}
