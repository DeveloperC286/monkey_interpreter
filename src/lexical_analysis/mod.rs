use std::iter::{FromIterator, Peekable};
use std::str::Chars;

use token::{Token, TokenType};

use crate::utilities::KEYWORDS;

pub mod token;
#[macro_use]
mod macros;

pub fn get_tokens(code: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut iterator = code.chars().peekable();

    loop {
        let (returned_iterator, token) = get_next_token(iterator);
        tokens.push(token.clone());
        iterator = returned_iterator;

        if token.token_type == TokenType::EOF {
            break;
        }
    }

    return tokens;
}

fn get_next_token(mut iterator: Peekable<Chars>) -> (Peekable<Chars>, Token) {
    let (returned_iterator, character) = get_next_character(iterator);
    iterator = returned_iterator;
    match character {
        Some(character) => {
            debug!("Matching the character '{}'.", character);
            match character {
                '!' => {
                    check_next_character!(iterator, '=', "!=", TokenType::NOT_EQUALS);
                    return_token!(iterator, '!', TokenType::NOT);
                }
                '-' => return_token!(iterator, '-', TokenType::MINUS),
                '/' => return_token!(iterator, '/', TokenType::DIVIDE),
                '*' => return_token!(iterator, '*', TokenType::MULTIPLY),
                '>' => return_token!(iterator, '>', TokenType::GREATER_THAN),
                '<' => return_token!(iterator, '<', TokenType::LESSER_THAN),
                '=' => {
                    check_next_character!(iterator, '=', "==", TokenType::EQUALS);
                    return_token!(iterator, '=', TokenType::ASSIGN);
                }
                '+' => return_token!(iterator, '+', TokenType::PLUS),
                '(' => return_token!(iterator, '(', TokenType::OPENING_ROUND_BRACKET),
                ')' => return_token!(iterator, ')', TokenType::CLOSING_ROUND_BRACKET),
                '{' => return_token!(iterator, '{', TokenType::OPENING_CURLY_BRACKET),
                '}' => return_token!(iterator, '}', TokenType::CLOSING_CURLY_BRACKET),
                ',' => return_token!(iterator, ',', TokenType::COMMA),
                ';' => return_token!(iterator, ';', TokenType::SEMI_COLON),
                _ => {
                    if is_valid_identifier_character(character) {
                        debug!("Parsing word from characters.");
                        let (returned_iterator, word) = get_word(iterator, character);
                        let word_lowercase = word.to_lowercase();
                        let mut token_type = TokenType::IDENTIFIER;

                        if KEYWORDS.contains_key(&word_lowercase) {
                            token_type = *KEYWORDS.get(&word_lowercase).unwrap();
                        }

                        return_token!(returned_iterator, word, token_type);
                    }

                    if is_digit(character) {
                        debug!("Parsing integer from characters.");
                        let (returned_iterator, integer) = get_integer(iterator, character);

                        return_token!(returned_iterator, integer, TokenType::INTEGER);
                    }

                    return_token!(iterator, character, TokenType::ILLEGAL);
                }
            }
        }
        None => {
            return_token!(iterator, "", TokenType::EOF);
        }
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

    match next_character {
        Some(character) => match character {
            ' ' | '\t' | '\n' | '\r' => {
                trace!("Consuming formatting character.");
                let (returned_iterator, returned_next_character) = get_next_character(iterator);
                iterator = returned_iterator;
                next_character = returned_next_character;
            }
            _ => {}
        },
        None => {}
    }

    return (iterator, next_character);
}

fn is_digit(character: char) -> bool {
    character.is_digit(10)
}

fn is_valid_identifier_character(character: char) -> bool {
    character.is_alphabetic() || character == '_'
}

#[cfg(test)]
mod tests;
