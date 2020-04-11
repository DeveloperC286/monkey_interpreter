use super::lexical_analysis::LexicalAnalysis;
use super::token::TokenType;
use std::io::{self};

pub fn repl() {
    loop {
        let input = read();
        let mut lexical_analysis = LexicalAnalysis::new(input);
        let mut token = lexical_analysis.get_next_token();

        while token.token_type != TokenType::EOF {
            info!("> {:?}", token);
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
