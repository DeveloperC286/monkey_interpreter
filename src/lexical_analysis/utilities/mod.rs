use crate::lexical_analysis::token::Token;

pub fn get_keyword_token(keyword: String) -> Token {
    match keyword.to_lowercase().as_str() {
        "fn" => Token::FUNCTION,
        "let" => Token::LET,
        "true" => Token::TRUE,
        "false" => Token::FALSE,
        "if" => Token::IF,
        "else" => Token::ELSE,
        "return" => Token::RETURN,
        _ => Token::IDENTIFIER { literal: keyword },
    }
}
