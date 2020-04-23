use super::lexical_analysis::token::{Token, TokenType};
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
        let tokenized_code = get_tokens(read());
        println!("{:?}", tokenized_code);
        let ast = get_abstract_syntax_tree(tokenized_code);
        println!("{:?}", ast);
    }
}

fn get_abstract_syntax_tree(tokenized_code: Vec<Token>) -> AbstractSyntaxTree {
    let mut syntax_analysis = SyntaxAnalysis::new();
    return syntax_analysis.parse(tokenized_code);
}

fn get_tokens(input: String) -> Vec<Token> {
    let mut lexical_analysis = LexicalAnalysis::new(input);
    let mut token = lexical_analysis.get_next_token();
    let mut tokenized_code: Vec<Token> = vec![];

    while token.token_type != TokenType::EOF {
        tokenized_code.push(token);
        token = lexical_analysis.get_next_token();
    }
    tokenized_code.push(token);

    return tokenized_code;
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
