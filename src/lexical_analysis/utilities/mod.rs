use crate::lexical_analysis::model::token::Token;

pub(crate) fn get_keyword_token(keyword: &str) -> Token {
    match keyword.to_lowercase().as_str() {
        "fn" => Token::Function,
        "let" => Token::Let,
        "true" => Token::True,
        "false" => Token::False,
        "if" => Token::If,
        "else" => Token::Else,
        "return" => Token::Return,
        _ => Token::Identifier {
            literal: keyword.to_string(),
        },
    }
}

pub(crate) fn is_digit(character: char) -> bool {
    character.is_digit(10)
}

pub(crate) fn is_valid_identifier_character(character: char) -> bool {
    character.is_alphabetic() || character == '_'
}
