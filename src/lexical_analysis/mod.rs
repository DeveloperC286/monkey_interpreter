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
        fn get_token_from(from: &str) -> Result<Token, LexicalError> {
            match (parse_to_keyword_or_identifier(from), parse_to_integer(from)) {
                (Some(token), None) => Ok(token),
                (None, Some(token)) => Ok(token),

                (_, _) => Err(LexicalError::UnparsableContext(from.to_string())),
            }
        }

        let mut tokens = Vec::new();
        let mut current_non_token_stream: String = String::new();

        loop {
            match self.source_code.next() {
                Some(next_character) => match next_character {
                    ' ' | '\t' | '\n' | '\r' => {
                        trace!("Consuming formatting character.");

                        if !current_non_token_stream.is_empty() {
                            tokens.push(get_token_from(&current_non_token_stream)?);
                            current_non_token_stream.clear();
                        }
                    }
                    _ => match self.get_next_token(&next_character)? {
                        Some(next_token) => {
                            if !current_non_token_stream.is_empty() {
                                tokens.push(get_token_from(&current_non_token_stream)?);
                                current_non_token_stream.clear();
                            }
                            tokens.push(next_token);
                        }
                        None => {
                            current_non_token_stream.push(next_character);
                        }
                    },
                },
                None => {
                    if !current_non_token_stream.is_empty() {
                        tokens.push(get_token_from(&current_non_token_stream)?);
                        current_non_token_stream.clear();
                    }
                    tokens.push(Token::EndOfFile);
                    break;
                }
            }
        }

        Ok(tokens)
    }

    fn get_next_token(&mut self, character: &char) -> Result<Option<Token>, LexicalError> {
        debug!("Matching the character '{}'.", character);
        match character {
            '!' => {
                check_next_character!(self.source_code, '=', Ok(Some(Token::NotEquals)));
                Ok(Some(Token::Not))
            }
            '-' => Ok(Some(Token::Minus)),
            '/' => Ok(Some(Token::Divide)),
            '*' => Ok(Some(Token::Multiply)),
            '>' => Ok(Some(Token::GreaterThan)),
            '<' => Ok(Some(Token::LesserThan)),
            '=' => {
                check_next_character!(self.source_code, '=', Ok(Some(Token::Equals)));
                Ok(Some(Token::Assign))
            }
            '+' => Ok(Some(Token::Plus)),
            '(' => Ok(Some(Token::OpeningRoundBracket)),
            ')' => Ok(Some(Token::ClosingRoundBracket)),
            '{' => Ok(Some(Token::OpeningCurlyBracket)),
            '}' => Ok(Some(Token::ClosingCurlyBracket)),
            ',' => Ok(Some(Token::Comma)),
            ';' => Ok(Some(Token::SemiColon)),
            '"' => Ok(Some(Token::String {
                literal: self.get_string()?,
            })),
            _ => Ok(None),
        }
    }

    fn get_string(&mut self) -> Result<String, LexicalError> {
        let mut characters = vec![];

        loop {
            match self.source_code.next() {
                Some('"') => break,
                Some('\\') => match self.source_code.next() {
                    Some('\\') => characters.push('\\'),
                    Some('\'') => characters.push('\''),
                    Some('"') => characters.push('"'),
                    Some('t') => characters.push('\t'),
                    Some('n') => characters.push('\n'),
                    Some('r') => characters.push('\r'),
                    Some(character) => return Err(LexicalError::IllegalEscaping(character)),
                    None => return Err(LexicalError::StringNotClosed),
                },
                Some(character) => characters.push(character),
                None => return Err(LexicalError::StringNotClosed),
            }
        }

        Ok(String::from_iter(characters.iter()))
    }
}
