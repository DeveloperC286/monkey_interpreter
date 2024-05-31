use crate::lexical_analysis::model::token::Token;

pub(super) fn parse_keyword(parsing: &str) -> Option<Token> {
    match parsing.to_lowercase().as_str() {
        "fn" => Some(Token::Function),
        "let" => Some(Token::Let),
        "true" => Some(Token::True),
        "false" => Some(Token::False),
        "if" => Some(Token::If),
        "else" => Some(Token::Else),
        "return" => Some(Token::Return),
        _ => None,
    }
}

pub(super) fn parse_integer(parsing: &str) -> Option<Token> {
    match parsing.parse() {
        Ok(integer) => Some(Token::Integer { literal: integer }),
        Err(_) => None,
    }
}

pub(super) fn parse_identifier(parsing: &str) -> Option<Token> {
    if is_valid_identifier(parsing) {
        Some(Token::Identifier {
            literal: parsing.to_string(),
        })
    } else {
        None
    }
}

fn is_valid_identifier(verifying: &str) -> bool {
    verifying
        .chars()
        .map(is_valid_identifier_character)
        .filter(|results| !(*results))
        .count()
        == 0
}

fn is_valid_identifier_character(character: char) -> bool {
    character.is_alphabetic() || character == '_'
}
