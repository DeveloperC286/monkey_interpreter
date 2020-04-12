use std::iter::FromIterator;

pub mod token;
use token::{Token, TokenType};

use std::collections::HashMap;

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

pub struct LexicalAnalysis {
    code: String,
    current_index: i32,
    current_character: Option<char>,
    next_character: Option<char>,
}

impl LexicalAnalysis {
    pub fn new(code: String) -> LexicalAnalysis {
        return LexicalAnalysis {
            code: code,
            current_index: -1,
            current_character: None,
            next_character: None,
        };
    }

    pub fn get_next_token(&mut self) -> Token {
        self.get_next_character();

        match self.current_character {
            Some(character) => {
                debug!(
                    "Matching the character '{}' at index {}.",
                    character, self.current_index
                );
                match character {
                    '!' => {
                        match self.next_character {
                            Some(next_character) => match next_character {
                                '=' => {
                                    self.increment_character_index();
                                    debug!("Returning TokenType::NOT_EQUALS.");
                                    return Token {
                                        token_type: TokenType::NOT_EQUALS,
                                        literal: "!=".to_string(),
                                    };
                                }
                                _ => {}
                            },
                            None => {}
                        }

                        debug!("Returning TokenType::NOT.");
                        return Token {
                            token_type: TokenType::NOT,
                            literal: "!".to_string(),
                        };
                    }
                    '-' => {
                        debug!("Returning TokenType::MINUS.");
                        return Token {
                            token_type: TokenType::MINUS,
                            literal: "-".to_string(),
                        };
                    }
                    '/' => {
                        debug!("Returning TokenType::DIVIDE.");
                        return Token {
                            token_type: TokenType::DIVIDE,
                            literal: "/".to_string(),
                        };
                    }
                    '*' => {
                        debug!("Returning TokenType::MULTIPLY.");
                        return Token {
                            token_type: TokenType::MULTIPLY,
                            literal: "*".to_string(),
                        };
                    }
                    '>' => {
                        debug!("Returning TokenType::GREATER_THAN.");
                        return Token {
                            token_type: TokenType::GREATER_THAN,
                            literal: ">".to_string(),
                        };
                    }
                    '<' => {
                        debug!("Returning TokenType::LESSER_THAN.");
                        return Token {
                            token_type: TokenType::LESSER_THAN,
                            literal: "<".to_string(),
                        };
                    }
                    '=' => {
                        match self.next_character {
                            Some(next_character) => match next_character {
                                '=' => {
                                    self.increment_character_index();
                                    debug!("Returning TokenType::EQUALS.");
                                    return Token {
                                        token_type: TokenType::EQUALS,
                                        literal: "==".to_string(),
                                    };
                                }
                                _ => {}
                            },
                            None => {}
                        }

                        debug!("Returning TokenType::ASSIGN.");
                        return Token {
                            token_type: TokenType::ASSIGN,
                            literal: "=".to_string(),
                        };
                    }
                    '+' => {
                        debug!("Returning TokenType::PLUS.");
                        return Token {
                            token_type: TokenType::PLUS,
                            literal: "+".to_string(),
                        };
                    }
                    '(' => {
                        debug!("Returning TokenType::OPENING_ROUND_BRACKET.");
                        return Token {
                            token_type: TokenType::OPENING_ROUND_BRACKET,
                            literal: "(".to_string(),
                        };
                    }
                    ')' => {
                        debug!("Returning TokenType::CLOSING_ROUND_BRACKET.");
                        return Token {
                            token_type: TokenType::CLOSING_ROUND_BRACKET,
                            literal: ")".to_string(),
                        };
                    }
                    '{' => {
                        debug!("Returning TokenType::OPENING_CURLY_BRACKET.");
                        return Token {
                            token_type: TokenType::OPENING_CURLY_BRACKET,
                            literal: "{".to_string(),
                        };
                    }
                    '}' => {
                        debug!("Returning TokenType::CLOSING_CURLY_BRACKET.");
                        return Token {
                            token_type: TokenType::CLOSING_CURLY_BRACKET,
                            literal: "}".to_string(),
                        };
                    }
                    ',' => {
                        debug!("Returning TokenType::COMMA.");
                        return Token {
                            token_type: TokenType::COMMA,
                            literal: ",".to_string(),
                        };
                    }
                    ';' => {
                        debug!("Returning TokenType::SEMI_COLON.");
                        return Token {
                            token_type: TokenType::SEMI_COLON,
                            literal: ";".to_string(),
                        };
                    }
                    _ => {
                        if character.is_alphabetic() {
                            debug!("Parsing word from characters.");
                            let word = self.get_word();
                            let word_lowercase = word.to_lowercase();
                            let mut token_type = TokenType::IDENTIFIER;

                            if KEYWORDS.contains_key(&word_lowercase) {
                                token_type = *KEYWORDS.get(&word_lowercase).unwrap();
                            }

                            debug!("Returning TokenType::{:?}.", token_type);
                            return Token {
                                token_type,
                                literal: word,
                            };
                        }

                        if character.is_digit(10) {
                            debug!("Parsing integer from characters.");
                            let integer = self.get_integer();

                            debug!("Returning TokenType::INTEGER.");
                            return Token {
                                token_type: TokenType::INTEGER,
                                literal: integer,
                            };
                        }

                        debug!("Returning TokenType::ILLEGAL.");
                        return Token {
                            token_type: TokenType::ILLEGAL,
                            literal: character.to_string(),
                        };
                    }
                }
            }
            None => {
                debug!("Returning TokenType::EOF.");
                return Token {
                    token_type: TokenType::EOF,
                    literal: "".to_string(),
                };
            }
        }
    }

    fn get_integer(&mut self) -> String {
        let mut chars: Vec<char> = vec![];

        loop {
            match self.current_character {
                Some(character) => {
                    if character.is_digit(10) {
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
            match self.next_character {
                Some(character) => {
                    if character.is_digit(10) {
                        self.increment_character_index();
                    } else {
                        break;
                    }
                }
                None => {
                    break;
                }
            }
        }

        let integer = String::from_iter(chars.iter());
        debug!("Found integer '{}'.", integer);
        return integer;
    }

    fn get_word(&mut self) -> String {
        let mut chars: Vec<char> = vec![];

        loop {
            match self.current_character {
                Some(character) => {
                    if character.is_alphabetic() {
                        chars.push(character);
                    } else {
                        error!("self.current_character is not alphabetic, should never be able to get here.");
                        break;
                    }
                }
                None => {
                    error!("self.current_character is None, should never be able to get here.");
                    break;
                }
            }
            match self.next_character {
                Some(character) => {
                    if character.is_alphabetic() {
                        self.increment_character_index();
                    } else {
                        break;
                    }
                }
                None => {
                    break;
                }
            }
        }

        let word = String::from_iter(chars.iter());
        debug!("Found word '{}'.", word);
        return word;
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

#[cfg(test)]
mod tests;
