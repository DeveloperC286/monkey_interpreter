use std::iter::{FromIterator, Peekable};
use std::str::Chars;

use crate::lexical_analysis::model::token::Token;
use crate::lexical_analysis::utilities::*;

#[macro_use]
mod macros;

pub mod model;
mod utilities;

pub struct LexicalAnalysis<'a> {
    source_code: Peekable<Chars<'a>>,
}

impl<'a> LexicalAnalysis<'a> {
    pub fn from(code: &str) -> Vec<Token> {
        let mut lexical_analysis = LexicalAnalysis {
            source_code: code.chars().peekable(),
        };

        lexical_analysis.get_tokens()
    }

    fn get_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        loop {
            let token = self.get_next_token();
            let end_of_file = token == Token::EndOfFile;
            tokens.push(token);

            if end_of_file {
                break;
            }
        }

        tokens
    }

    fn get_next_token(&mut self) -> Token {
        match self.get_next_character() {
            Some(character) => {
                debug!("Matching the character '{}'.", character);
                match character {
                    '!' => {
                        check_next_character!(self.source_code, '=', Token::NotEquals);
                        Token::Not
                    }
                    '-' => Token::Minus,
                    '/' => Token::Divide,
                    '*' => Token::Multiply,
                    '>' => Token::GreaterThan,
                    '<' => Token::LesserThan,
                    '=' => {
                        check_next_character!(self.source_code, '=', Token::Equals);
                        Token::Assign
                    }
                    '+' => Token::Plus,
                    '(' => Token::OpeningRoundBracket,
                    ')' => Token::ClosingRoundBracket,
                    '{' => Token::OpeningCurlyBracket,
                    '}' => Token::ClosingCurlyBracket,
                    ',' => Token::Comma,
                    ';' => Token::SemiColon,
                    _ => {
                        if is_valid_identifier_character(character) {
                            debug!("Parsing word from characters.");
                            return get_keyword_token(&self.get_word(character));
                        }

                        if is_digit(character) {
                            debug!("Parsing integer from characters.");
                            return Token::Integer {
                                literal: self.get_integer(character),
                            };
                        }

                        Token::Illegal {
                            literal: character.to_string(),
                        }
                    }
                }
            }
            None => Token::EndOfFile,
        }
    }

    fn get_integer(&mut self, character: char) -> String {
        parse_characters!(self.source_code, character, is_digit);
    }

    fn get_word(&mut self, character: char) -> String {
        parse_characters!(self.source_code, character, is_valid_identifier_character);
    }

    fn get_next_character(&mut self) -> Option<char> {
        trace!("Getting next character.");
        let mut next_character = self.source_code.next();

        if let Some(character) = next_character {
            match character {
                ' ' | '\t' | '\n' | '\r' => {
                    trace!("Consuming formatting character.");
                    next_character = self.get_next_character();
                }
                _ => {}
            }
        }

        next_character
    }
}

#[cfg(test)]
mod tests;
