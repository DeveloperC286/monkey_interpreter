use std::iter::Peekable;
use std::str::Chars;

use log::{debug, info, trace};

mod token;

pub use token::Token;

pub(crate) struct LexicalAnalysis<'a> {
    source_code: Peekable<Chars<'a>>,
}

impl LexicalAnalysis<'_> {
    pub(crate) fn from(code: &str) -> anyhow::Result<Vec<Token>> {
        let mut lexical_analysis = LexicalAnalysis {
            source_code: code.chars().peekable(),
        };

        lexical_analysis.parse_source_code()
    }

    fn parse_source_code(&mut self) -> anyhow::Result<Vec<Token>> {
        fn parse_context(context: &str) -> anyhow::Result<Token> {
            match (parse_integer(context), parse_identifier(context)) {
                (IntegerParseOutcome::Valid(integer), IdentifierParseOutcome::NotAnIdentifier) => {
                    Ok(integer)
                }
                (IntegerParseOutcome::OutOfRange, IdentifierParseOutcome::NotAnIdentifier) => {
                    anyhow::bail!(
                        "The integer literal {:?} does not fit into a signed 64 bit integer.",
                        context
                    )
                }
                (IntegerParseOutcome::NotAnInteger, IdentifierParseOutcome::Keyword(keyword)) => {
                    Ok(keyword)
                }
                (
                    IntegerParseOutcome::NotAnInteger,
                    IdentifierParseOutcome::Identifier(identifier),
                ) => Ok(identifier),
                (_, _) => {
                    anyhow::bail!("Unparsable context for lexical analysis {:?}.", context)
                }
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

    fn parse_character(&mut self, character: &char) -> anyhow::Result<Option<Token>> {
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

    fn parse_string_object(&mut self) -> anyhow::Result<String> {
        trace!("Attempting to parse a string object.");
        let mut string_object = String::new();

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
                    Some(character) => {
                        anyhow::bail!("Illegal escaping of the character {:?}.", character)
                    }
                    None => anyhow::bail!("String not closed before the end of the code."),
                },
                Some(character) => string_object.push(character),
                None => anyhow::bail!("String not closed before the end of the code."),
            }
        }

        trace!("Parsed the string object {string_object:?}.");
        Ok(string_object)
    }
}

enum IntegerParseOutcome {
    Valid(Token),
    OutOfRange,
    NotAnInteger,
}

fn parse_integer(parsing: &str) -> IntegerParseOutcome {
    match parsing.parse() {
        Ok(literal) => IntegerParseOutcome::Valid(Token::Integer { literal }),
        Err(error) => match error.kind() {
            std::num::IntErrorKind::PosOverflow | std::num::IntErrorKind::NegOverflow => {
                IntegerParseOutcome::OutOfRange
            }
            _ => IntegerParseOutcome::NotAnInteger,
        },
    }
}

enum IdentifierParseOutcome {
    Keyword(Token),
    Identifier(Token),
    NotAnIdentifier,
}

fn parse_identifier(parsing: &str) -> IdentifierParseOutcome {
    if !is_valid_identifier(parsing) {
        return IdentifierParseOutcome::NotAnIdentifier;
    }

    match parsing.to_lowercase().as_str() {
        "fn" => IdentifierParseOutcome::Keyword(Token::Function),
        "let" => IdentifierParseOutcome::Keyword(Token::Let),
        "true" => IdentifierParseOutcome::Keyword(Token::True),
        "false" => IdentifierParseOutcome::Keyword(Token::False),
        "if" => IdentifierParseOutcome::Keyword(Token::If),
        "else" => IdentifierParseOutcome::Keyword(Token::Else),
        "return" => IdentifierParseOutcome::Keyword(Token::Return),
        _ => IdentifierParseOutcome::Identifier(Token::Identifier {
            literal: parsing.to_string(),
        }),
    }
}

fn is_valid_identifier(verifying: &str) -> bool {
    let mut characters = verifying.chars();

    let starts_valid = match characters.next() {
        Some(character) => is_valid_identifier_start_character(character),
        None => false,
    };

    starts_valid && characters.all(is_valid_identifier_character)
}

fn is_valid_identifier_start_character(character: char) -> bool {
    character.is_alphabetic() || character == '_'
}

fn is_valid_identifier_character(character: char) -> bool {
    is_valid_identifier_start_character(character) || character.is_ascii_digit()
}
