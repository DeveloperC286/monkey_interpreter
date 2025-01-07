use std::iter::{FromIterator, Peekable};
use std::str::Chars;

use crate::lexical_analysis::model::lexical_error::LexicalError;
use crate::lexical_analysis::model::token::Token;

pub(crate) mod model;

pub(crate) struct LexicalAnalysis<'a> {
    source_code: Peekable<Chars<'a>>,
}

impl LexicalAnalysis<'_> {
    pub(crate) fn from(code: &str) -> Result<Vec<Token>, LexicalError> {
        let mut lexical_analysis = LexicalAnalysis {
            source_code: code.chars().peekable(),
        };

        lexical_analysis.parse_source_code()
    }

    fn parse_source_code(&mut self) -> Result<Vec<Token>, LexicalError> {
        fn parse_context(context: &str) -> Result<Token, LexicalError> {
            match (
                parse_integer(context),
                parse_keyword(context),
                parse_identifier(context),
            ) {
                (Some(integer), None, None) => Ok(integer),
                // When it is a valid keyword and identifier, then it is a keyword.
                (None, Some(keyword), _) => Ok(keyword),
                (None, None, Some(identifier)) => Ok(identifier),
                (_, _, _) => Err(LexicalError::UnparsableContext(context.to_string())),
            }
        }

        macro_rules! add_token {
            ($tokens:expr, $token:expr) => {
                debug!("Parsed the token '{:?}'.", $token);
                $tokens.push($token);
            };
        }

        macro_rules! parse_context {
            ($tokens:expr, $context:expr) => {
                if !$context.is_empty() {
                    trace!("Attempting to parse the context {:?} to a token.", $context);
                    add_token!($tokens, parse_context(&$context)?);
                    $context.clear();
                }
            };
        }

        let mut tokens = Vec::new();
        let mut context: String = String::new();

        info!("Starting Lexical Analysis.");
        loop {
            match self.source_code.next() {
                Some(character) => match character {
                    ' ' | '\t' | '\n' | '\r' => {
                        trace!("Consuming the formatting character {character:?}.");
                        parse_context!(tokens, context);
                    }
                    _ => match self.parse_character(&character)? {
                        Some(token) => {
                            parse_context!(tokens, context);
                            add_token!(tokens, token);
                        }
                        None => {
                            context.push(character);
                        }
                    },
                },
                None => {
                    debug!("End of the source code.");
                    parse_context!(tokens, context);
                    break;
                }
            }
        }

        debug!("Parsed the following tokens from the source code {tokens:?}.");
        info!("End of Lexical Analysis.");
        Ok(tokens)
    }

    fn parse_character(&mut self, character: &char) -> Result<Option<Token>, LexicalError> {
        trace!("Attempting to parse the character {character:?} to a token.");
        match character {
            '!' => match self.source_code.peek() {
                Some('=') => {
                    self.source_code.next();
                    Ok(Some(Token::NotEquals))
                }
                _ => Ok(Some(Token::Not)),
            },
            '-' => Ok(Some(Token::Minus)),
            '/' => Ok(Some(Token::Divide)),
            '*' => Ok(Some(Token::Multiply)),
            '>' => Ok(Some(Token::GreaterThan)),
            '<' => Ok(Some(Token::LesserThan)),
            '=' => match self.source_code.peek() {
                Some('=') => {
                    self.source_code.next();
                    Ok(Some(Token::Equals))
                }
                _ => Ok(Some(Token::Assign)),
            },
            '+' => Ok(Some(Token::Plus)),
            '(' => Ok(Some(Token::OpeningRoundBracket)),
            ')' => Ok(Some(Token::ClosingRoundBracket)),
            '{' => Ok(Some(Token::OpeningCurlyBracket)),
            '}' => Ok(Some(Token::ClosingCurlyBracket)),
            ',' => Ok(Some(Token::Comma)),
            ';' => Ok(Some(Token::SemiColon)),
            '"' => Ok(Some(Token::String {
                literal: self.parse_string_object()?,
            })),
            _ => {
                trace!("Unable to parse the character {character:?} to a token.");
                Ok(None)
            }
        }
    }

    fn parse_string_object(&mut self) -> Result<String, LexicalError> {
        trace!("Attempting to parse a string object.");
        let mut string_object = vec![];

        loop {
            match self.source_code.next() {
                Some('"') => break,
                Some('\\') => match self.source_code.next() {
                    Some('\\') => string_object.push('\\'),
                    Some('\'') => string_object.push('\''),
                    Some('"') => string_object.push('"'),
                    Some('t') => string_object.push('\t'),
                    Some('n') => string_object.push('\n'),
                    Some('r') => string_object.push('\r'),
                    Some(character) => return Err(LexicalError::IllegalEscaping(character)),
                    None => return Err(LexicalError::StringNotClosed),
                },
                Some(character) => string_object.push(character),
                None => return Err(LexicalError::StringNotClosed),
            }
        }

        let string = String::from_iter(string_object.iter());
        trace!("Parsed the string object {string:?}.");
        Ok(string)
    }
}

fn parse_keyword(parsing: &str) -> Option<Token> {
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

fn parse_integer(parsing: &str) -> Option<Token> {
    match parsing.parse() {
        Ok(integer) => Some(Token::Integer { literal: integer }),
        Err(_) => None,
    }
}

fn parse_identifier(parsing: &str) -> Option<Token> {
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
