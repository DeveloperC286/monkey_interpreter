use std::iter::{FromIterator, Peekable};
use std::str::Chars;

use crate::lexical_analysis::model::token::Token;
use crate::lexical_analysis::utilities::*;

#[macro_use]
mod macros;

pub(crate) mod model;
mod utilities;

pub fn get_tokens(code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut iterator = code.chars().peekable();

    loop {
        let (returned_iterator, token) = get_next_token(iterator);
        tokens.push(token.clone());
        iterator = returned_iterator;

        if token == Token::EndOfFile {
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
                    check_next_character!(iterator, '=', Token::NotEquals);
                    (iterator, Token::Not)
                }
                '-' => (iterator, Token::Minus),
                '/' => (iterator, Token::Divide),
                '*' => (iterator, Token::Multiply),
                '>' => (iterator, Token::GreaterThan),
                '<' => (iterator, Token::LesserThan),
                '=' => {
                    check_next_character!(iterator, '=', Token::Equals);
                    (iterator, Token::Assign)
                }
                '+' => (iterator, Token::Plus),
                '(' => (iterator, Token::OpeningRoundBracket),
                ')' => (iterator, Token::ClosingRoundBracket),
                '{' => (iterator, Token::OpeningCurlyBracket),
                '}' => (iterator, Token::ClosingCurlyBracket),
                ',' => (iterator, Token::Comma),
                ';' => (iterator, Token::SemiColon),
                _ => {
                    if is_valid_identifier_character(character) {
                        debug!("Parsing word from characters.");
                        let (returned_iterator, word) = get_word(iterator, character);

                        return (returned_iterator, get_keyword_token(&word));
                    }

                    if is_digit(character) {
                        debug!("Parsing integer from characters.");
                        let (returned_iterator, integer) = get_integer(iterator, character);

                        return (returned_iterator, Token::Integer { literal: integer });
                    }

                    (
                        iterator,
                        Token::Illegal {
                            literal: character.to_string(),
                        },
                    )
                }
            }
        }
        None => (iterator, Token::EndOfFile),
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

#[cfg(test)]
mod tests;
