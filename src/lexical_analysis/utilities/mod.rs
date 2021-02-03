use crate::lexical_analysis::model::token::Token;

pub fn get_keyword_token(keyword: &str) -> Token {
    match keyword.to_lowercase().as_str() {
        "fn" => Token::FUNCTION,
        "let" => Token::LET,
        "true" => Token::TRUE,
        "false" => Token::FALSE,
        "if" => Token::IF,
        "else" => Token::ELSE,
        "return" => Token::RETURN,
        _ => Token::IDENTIFIER {
            literal: keyword.to_string(),
        },
    }
}

pub fn is_digit(character: char) -> bool {
    character.is_digit(10)
}

pub fn is_valid_identifier_character(character: char) -> bool {
    character.is_alphabetic() || character == '_'
}
