pub mod token;
use token::{Token, TokenType};

use std::collections::HashMap;
use std::iter::FromIterator;

lazy_static! {
    static ref KEYWORDS: HashMap<String, TokenType> = {
        let mut m = HashMap::new();
        m.insert("fn".to_string(), TokenType::FUNCTION);
        m.insert("let".to_string(), TokenType::LET);
        m.insert("true".to_string(), TokenType::TRUE);
        m.insert("false".to_string(), TokenType::FALSE);
        m.insert("if".to_string(), TokenType::IF);
        m.insert("else".to_string(), TokenType::ELSE);
        m.insert("return".to_string(), TokenType::RETURN);
        m
    };
}

macro_rules! parse_characters {
    ($self:expr, $valid_character:ident) => {
        let mut chars: Vec<char> = vec![];

        loop {
            match $self.current_character {
                Some(character) => {
                    if $valid_character(character) {
                        chars.push(character);
                    } else {
                        error!("self.current_character is not a digit, should never be able to get here.");
                        break;
                    }
                }
                None => {
                    error!("self.current_character is None, should never be able to get here.");
                    break;
                }
            }
            match $self.next_character {
                Some(character) => {
                    if $valid_character(character) {
                        $self.increment_character_index();
                    } else {
                        break;
                    }
                }
                None => {
                    break;
                }
            }
        }

        let string = String::from_iter(chars.iter());
        return string;
    }
}

macro_rules! check_next_character {
    ($self:expr, $expected_next_character:expr, $literal:expr, $token_type:expr) => {
        match $self.next_character {
            Some(next_character) => match next_character {
                $expected_next_character => {
                    $self.increment_character_index();
                    return_token!($literal, $token_type);
                }
                _ => {}
            },
            None => {}
        }
    };
}

macro_rules! return_token {
    ($literal:expr, $token_type:expr) => {
        return Token {
            token_type: $token_type,
            literal: $literal.to_string(),
        };
    };
}

pub struct LexicalAnalysis {
    code: String,
    current_index: i32,
    current_character: Option<char>,
    next_character: Option<char>,
}

impl LexicalAnalysis {
    pub fn new() -> LexicalAnalysis {
        return LexicalAnalysis {
            code: "".to_string(),
            current_index: -1,
            current_character: None,
            next_character: None,
        };
    }

    pub fn get_tokens(&mut self, code: String) -> Vec<Token> {
        let mut tokens = Vec::new();
        self.code = code;
        self.current_index = -1;
        self.current_character = None;
        self.next_character = None;

        loop {
            let token = self.get_next_token();

            if token.token_type == TokenType::EOF {
                tokens.push(token);
                break;
            } else {
                tokens.push(token);
            }
        }

        return tokens;
    }

    fn get_next_token(&mut self) -> Token {
        self.get_next_character();

        match self.current_character {
            Some(character) => {
                debug!(
                    "Matching the character '{}' at index {}.",
                    character, self.current_index
                );
                match character {
                    '!' => {
                        check_next_character!(self, '=', "!=", TokenType::NOT_EQUALS);
                        return_token!('!', TokenType::NOT);
                    }
                    '-' => return_token!('-', TokenType::MINUS),
                    '/' => return_token!('/', TokenType::DIVIDE),
                    '*' => return_token!('*', TokenType::MULTIPLY),
                    '>' => return_token!('>', TokenType::GREATER_THAN),
                    '<' => return_token!('<', TokenType::LESSER_THAN),
                    '=' => {
                        check_next_character!(self, '=', "==", TokenType::EQUALS);
                        return_token!('=', TokenType::ASSIGN);
                    }
                    '+' => return_token!('+', TokenType::PLUS),
                    '(' => return_token!('(', TokenType::OPENING_ROUND_BRACKET),
                    ')' => return_token!(')', TokenType::CLOSING_ROUND_BRACKET),
                    '{' => return_token!('{', TokenType::OPENING_CURLY_BRACKET),
                    '}' => return_token!('}', TokenType::CLOSING_CURLY_BRACKET),
                    ',' => return_token!(',', TokenType::COMMA),
                    ';' => return_token!(';', TokenType::SEMI_COLON),
                    _ => {
                        if is_valid_identifer_character(character) {
                            debug!("Parsing word from characters.");
                            let word = self.get_word();
                            let word_lowercase = word.to_lowercase();
                            let mut token_type = TokenType::IDENTIFIER;

                            if KEYWORDS.contains_key(&word_lowercase) {
                                token_type = *KEYWORDS.get(&word_lowercase).unwrap();
                            }

                            return_token!(word, token_type);
                        }

                        if is_digit(character) {
                            debug!("Parsing integer from characters.");
                            let integer = self.get_integer();

                            return_token!(integer, TokenType::INTEGER);
                        }

                        return_token!(character, TokenType::ILLEGAL);
                    }
                }
            }
            None => {
                return_token!("", TokenType::EOF);
            }
        }
    }

    fn get_integer(&mut self) -> String {
        parse_characters!(self, is_digit);
    }

    fn get_word(&mut self) -> String {
        parse_characters!(self, is_valid_identifer_character);
    }

    fn get_next_character(&mut self) {
        trace!("Getting next character.");
        self.increment_character_index();

        match self.current_character {
            Some(character) => match character {
                ' ' | '\t' | '\n' | '\r' => {
                    trace!("Consuming formatting character.");
                    self.get_next_character()
                }
                _ => {}
            },
            None => {}
        }
    }

    fn increment_character_index(&mut self) {
        trace!("Incrementing character index.");
        self.current_index += 1;
        self.current_character = self.code.chars().nth(self.current_index as usize);
        self.next_character = self.code.chars().nth((self.current_index + 1) as usize);
    }
}

fn is_digit(character: char) -> bool {
    character.is_digit(10)
}

fn is_valid_identifer_character(character: char) -> bool {
    character.is_alphabetic() || character == '_'
}

#[cfg(test)]
mod tests;
