use super::lexical_analysis::token::Token;
use super::lexical_analysis::LexicalAnalysis;
use super::syntax_analysis::abstract_syntax_tree::AbstractSyntaxTree;
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
        let ast = get_abstract_syntax_tree(tokens);
        println!("{:?}", ast);
    }
}

fn get_abstract_syntax_tree(tokens: Vec<Token>) -> AbstractSyntaxTree {
    let mut syntax_analysis = SyntaxAnalysis::new();
    return syntax_analysis.parse(tokens);
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
