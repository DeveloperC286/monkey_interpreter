use std::iter::{FromIterator, Peekable};
use std::str::Chars;

use crate::lexical_analysis::model::lexical_error::LexicalError;
use crate::lexical_analysis::model::token::Token;
use crate::lexical_analysis::utilities::*;

#[macro_use]
mod macros;

pub(crate) mod model;
mod utilities;

pub(crate) struct LexicalAnalysis<'a> {
    source_code: Peekable<Chars<'a>>,
}

impl<'a> LexicalAnalysis<'a> {
    pub(crate) fn from(code: &str) -> Result<Vec<Token>, LexicalError> {
        let mut lexical_analysis = LexicalAnalysis {
            source_code: code.chars().peekable(),
        };

        lexical_analysis.get_tokens()
    }

    fn get_tokens(&mut self) -> Result<Vec<Token>, LexicalError> {
        let mut tokens = Vec::new();

        loop {
            let token = self.get_next_token()?;
            let end_of_file = token == Token::EndOfFile;
            tokens.push(token);

            if end_of_file {
                break;
            }
        }

        Ok(tokens)
    }

    fn get_next_token(&mut self) -> Result<Token, LexicalError> {
        match self.get_next_character() {
            Some(character) => {
                debug!("Matching the character '{}'.", character);
                match character {
                    '!' => {
                        check_next_character!(self.source_code, '=', Ok(Token::NotEquals));
                        Ok(Token::Not)
                    }
                    '-' => Ok(Token::Minus),
                    '/' => Ok(Token::Divide),
                    '*' => Ok(Token::Multiply),
                    '>' => Ok(Token::GreaterThan),
                    '<' => Ok(Token::LesserThan),
                    '=' => {
                        check_next_character!(self.source_code, '=', Ok(Token::Equals));
                        Ok(Token::Assign)
                    }
                    '+' => Ok(Token::Plus),
                    '(' => Ok(Token::OpeningRoundBracket),
                    ')' => Ok(Token::ClosingRoundBracket),
                    '{' => Ok(Token::OpeningCurlyBracket),
                    '}' => Ok(Token::ClosingCurlyBracket),
                    ',' => Ok(Token::Comma),
                    ';' => Ok(Token::SemiColon),
                    _ => {
                        if is_valid_identifier_character(character) {
                            debug!("Parsing word from characters.");
                            return Ok(get_keyword_token(&self.get_word(character)));
                        }

                        if is_digit(character) {
                            debug!("Parsing integer from characters.");
                            return Ok(Token::Integer {
                                literal: self.get_integer(character)?,
                            });
                        }

                        Err(LexicalError::IllegalCharacter(character))
                    }
                }
            }
            None => Ok(Token::EndOfFile),
        }
    }

    fn get_integer_string(&mut self, character: char) -> String {
        parse_characters!(self.source_code, character, is_digit);
    }

    fn get_integer(&mut self, character: char) -> Result<i64, LexicalError> {
        let integer_string = self.get_integer_string(character);

        match integer_string.parse() {
            Ok(integer) => Ok(integer),
            Err(_) => Err(LexicalError::NonNumericIntegerString(integer_string)),
        }
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
