use std::iter::{FromIterator, Peekable};
use std::str::Chars;

use token::Token;

pub mod token;
pub mod utilities;
#[macro_use]
mod macros;

pub fn get_tokens(code: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut iterator = code.chars().peekable();

    loop {
        let (returned_iterator, token) = get_next_token(iterator);
        tokens.push(token.clone());
        iterator = returned_iterator;

        if token == Token::EOF {
            break;
        }
    }

    tokens
}

fn get_next_token(mut iterator: Peekable<Chars>) -> (Peekable<Chars>, Token) {
    let (returned_iterator, character) = get_next_character(iterator);
    iterator = returned_iterator;
    match character {
        Some(character) => {
            debug!("Matching the character '{}'.", character);
            match character {
                '!' => {
                    check_next_character!(iterator, '=', Token::NOT_EQUALS);
                    (iterator, Token::NOT)
                }
                '-' => (iterator, Token::MINUS),
                '/' => (iterator, Token::DIVIDE),
                '*' => (iterator, Token::MULTIPLY),
                '>' => (iterator, Token::GREATER_THAN),
                '<' => (iterator, Token::LESSER_THAN),
                '=' => {
                    check_next_character!(iterator, '=', Token::EQUALS);
                    (iterator, Token::ASSIGN)
                }
                '+' => (iterator, Token::PLUS),
                '(' => (iterator, Token::OPENING_ROUND_BRACKET),
                ')' => (iterator, Token::CLOSING_ROUND_BRACKET),
                '{' => (iterator, Token::OPENING_CURLY_BRACKET),
                '}' => (iterator, Token::CLOSING_CURLY_BRACKET),
                ',' => (iterator, Token::COMMA),
                ';' => (iterator, Token::SEMI_COLON),
                _ => {
                    if is_valid_identifier_character(character) {
                        debug!("Parsing word from characters.");
                        let (returned_iterator, word) = get_word(iterator, character);

                        return (returned_iterator, utilities::get_keyword_token(word));
                    }

                    if is_digit(character) {
                        debug!("Parsing integer from characters.");
                        let (returned_iterator, integer) = get_integer(iterator, character);

                        return (returned_iterator, Token::INTEGER { literal: integer });
                    }

                    (
                        iterator,
                        Token::ILLEGAL {
                            literal: character.to_string(),
                        },
                    )
                }
            }
        }
        None => (iterator, Token::EOF),
    }
}

fn get_integer(mut iterator: Peekable<Chars>, character: char) -> (Peekable<Chars>, String) {
    parse_characters!(iterator, character, is_digit);
}

fn get_word(mut iterator: Peekable<Chars>, character: char) -> (Peekable<Chars>, String) {
    parse_characters!(iterator, character, is_valid_identifier_character);
}

fn get_next_character(mut iterator: Peekable<Chars>) -> (Peekable<Chars>, Option<char>) {
    trace!("Getting next character.");
    let mut next_character = iterator.next();

    if let Some(character) = next_character {
        match character {
            ' ' | '\t' | '\n' | '\r' => {
                trace!("Consuming formatting character.");
                let (returned_iterator, returned_next_character) = get_next_character(iterator);
                iterator = returned_iterator;
                next_character = returned_next_character;
            }
            _ => {}
        }
    }

    (iterator, next_character)
}

fn is_digit(character: char) -> bool {
    character.is_digit(10)
}

fn is_valid_identifier_character(character: char) -> bool {
    character.is_alphabetic() || character == '_'
}

#[cfg(test)]
mod tests;
