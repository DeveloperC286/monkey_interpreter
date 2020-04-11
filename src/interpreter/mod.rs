use super::lexical_analysis::token::TokenType;
use super::lexical_analysis::LexicalAnalysis;
use std::io::{self};

pub fn repl() {
    loop {
        let input = read();
        let mut lexical_analysis = LexicalAnalysis::new(input);
        let mut token = lexical_analysis.get_next_token();

        while token.token_type != TokenType::EOF {
            println!("> {:?}", token);
            token = lexical_analysis.get_next_token();
        }
    }
}

fn read() -> String {
    let mut buffer = String::new();

    match io::stdin().read_line(&mut buffer) {
        Ok(_n) => {}
        Err(error) => error!("Error reading user input: {}", error),
    }

    return buffer;
}
