use std::iter::Peekable;
use std::str::Chars;

use log::{debug, info, trace};

use crate::lexical_analysis::model::token::{Position, PositionedToken, Token};

pub(crate) mod model;

pub(crate) struct LexicalAnalysis<'a> {
    source_code: Peekable<Chars<'a>>,
    line: usize,
    column: usize,
}

impl LexicalAnalysis<'_> {
    pub(crate) fn from(code: &str) -> anyhow::Result<Vec<PositionedToken>> {
        let mut lexical_analysis = LexicalAnalysis {
            source_code: code.chars().peekable(),
            line: 1,
            column: 1,
        };

        lexical_analysis.parse_source_code()
    }

    fn position(&self) -> Position {
        Position {
            line: self.line,
            column: self.column,
        }
    }

    fn advance(&mut self) -> Option<char> {
        let character = self.source_code.next();

        if let Some(character) = character {
            if character == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }

        character
    }

    fn parse_source_code(&mut self) -> anyhow::Result<Vec<PositionedToken>> {
        fn parse_context(context: &str, position: Position) -> anyhow::Result<Token> {
            match (
                parse_integer(context),
                parse_keyword(context),
                parse_identifier(context),
            ) {
                (Some(integer), None, None) => Ok(integer),
                // When it is a valid keyword and identifier, then it is a keyword.
                (None, Some(keyword), _) => Ok(keyword),
                (None, None, Some(identifier)) => Ok(identifier),
                (_, _, _) => {
                    anyhow::bail!(
                        "Unparsable context for lexical analysis {:?} at {position}.",
                        context
                    )
                }
            }
        }

        macro_rules! add_token {
            ($tokens:expr, $token:expr, $position:expr) => {
                let positioned_token = PositionedToken {
                    token: $token,
                    position: $position,
                };
                debug!("Parsed the token '{:?}'.", positioned_token);
                $tokens.push(positioned_token);
            };
        }

        macro_rules! parse_context {
            ($tokens:expr, $context:expr, $context_position:expr) => {
                if !$context.is_empty() {
                    trace!("Attempting to parse the context {:?} to a token.", $context);
                    let context_position = $context_position
                        .take()
                        .expect("A non-empty context must have a recorded starting position.");
                    let token = parse_context(&$context, context_position)?;
                    add_token!($tokens, token, context_position);
                    $context.clear();
                }
            };
        }

        let mut tokens = Vec::new();
        let mut context: String = String::new();
        let mut context_position: Option<Position> = None;

        info!("Starting Lexical Analysis.");
        loop {
            let position = self.position();

            match self.advance() {
                Some(character) => match character {
                    ' ' | '\t' | '\n' | '\r' => {
                        trace!("Consuming the formatting character {character:?}.");
                        parse_context!(tokens, context, context_position);
                    }
                    _ => match self.parse_character(&character, position)? {
                        Some(token) => {
                            parse_context!(tokens, context, context_position);
                            add_token!(tokens, token, position);
                        }
                        None => {
                            if context.is_empty() {
                                context_position = Some(position);
                            }
                            context.push(character);
                        }
                    },
                },
                None => {
                    debug!("End of the source code.");
                    parse_context!(tokens, context, context_position);
                    break;
                }
            }
        }

        debug!("Parsed the following tokens from the source code {tokens:?}.");
        info!("End of Lexical Analysis.");
        Ok(tokens)
    }

    fn parse_character(
        &mut self,
        character: &char,
        position: Position,
    ) -> anyhow::Result<Option<Token>> {
        trace!("Attempting to parse the character {character:?} to a token.");
        match character {
            '!' => match self.source_code.peek() {
                Some('=') => {
                    self.advance();
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
                    self.advance();
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
                literal: self.parse_string_object(position)?,
            })),
            _ => {
                trace!("Unable to parse the character {character:?} to a token.");
                Ok(None)
            }
        }
    }

    fn parse_string_object(&mut self, start_position: Position) -> anyhow::Result<String> {
        trace!("Attempting to parse a string object.");
        let mut string_object = String::new();

        loop {
            match self.advance() {
                Some('"') => break,
                Some('\\') => match self.advance() {
                    Some('\\') => string_object.push('\\'),
                    Some('\'') => string_object.push('\''),
                    Some('"') => string_object.push('"'),
                    Some('t') => string_object.push('\t'),
                    Some('n') => string_object.push('\n'),
                    Some('r') => string_object.push('\r'),
                    Some(character) => {
                        anyhow::bail!(
                            "Illegal escaping of the character {:?} at {}.",
                            character,
                            self.position()
                        )
                    }
                    None => anyhow::bail!(
                        "String starting at {start_position} not closed before the end of the code."
                    ),
                },
                Some(character) => string_object.push(character),
                None => anyhow::bail!(
                    "String starting at {start_position} not closed before the end of the code."
                ),
            }
        }

        trace!("Parsed the string object {string_object:?}.");
        Ok(string_object)
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
    verifying.chars().all(is_valid_identifier_character)
}

fn is_valid_identifier_character(character: char) -> bool {
    character.is_alphabetic() || character == '_'
}
