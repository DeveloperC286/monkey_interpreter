use super::token::{Token, TokenType};
use std::iter::FromIterator;

use std::collections::HashMap;

lazy_static! {
    static ref KEYWORDS: HashMap<String, TokenType> = {
        let mut m = HashMap::new();
        m.insert("fn".to_string(), TokenType::FUNCTION);
        m.insert("let".to_string(), TokenType::LET);
        m
    };
}

pub struct LexicalAnalysis {
    pub code: String,
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
            Some(character) => match character {
                '-' => {
                    return Token {
                        token_type: TokenType::MINUS,
                        literal: "-".to_string(),
                    };
                }
                '/' => {
                    return Token {
                        token_type: TokenType::DIVIDE,
                        literal: "/".to_string(),
                    };
                }
                '*' => {
                    return Token {
                        token_type: TokenType::MULTIPLY,
                        literal: "*".to_string(),
                    };
                }
                '>' => {
                    return Token {
                        token_type: TokenType::GREATER_THAN,
                        literal: ">".to_string(),
                    };
                }
                '<' => {
                    return Token {
                        token_type: TokenType::LESSER_THAN,
                        literal: "<".to_string(),
                    };
                }
                '=' => {
                    return Token {
                        token_type: TokenType::ASSIGN,
                        literal: "=".to_string(),
                    };
                }
                '+' => {
                    return Token {
                        token_type: TokenType::PLUS,
                        literal: "+".to_string(),
                    };
                }
                '(' => {
                    return Token {
                        token_type: TokenType::OPENING_ROUND_BRACKET,
                        literal: "(".to_string(),
                    };
                }
                ')' => {
                    return Token {
                        token_type: TokenType::CLOSING_ROUND_BRACKET,
                        literal: ")".to_string(),
                    };
                }
                '{' => {
                    return Token {
                        token_type: TokenType::OPENING_CURLY_BRACKET,
                        literal: "{".to_string(),
                    };
                }
                '}' => {
                    return Token {
                        token_type: TokenType::CLOSING_CURLY_BRACKET,
                        literal: "}".to_string(),
                    };
                }
                ',' => {
                    return Token {
                        token_type: TokenType::COMMA,
                        literal: ",".to_string(),
                    };
                }
                ';' => {
                    return Token {
                        token_type: TokenType::SEMI_COLON,
                        literal: ";".to_string(),
                    };
                }
                _ => {
                    if character.is_alphabetic() {
                        let word = self.get_word();
                        let word_lowercase = word.to_lowercase();
                        let mut token_type = TokenType::IDENTIFIER;

                        if KEYWORDS.contains_key(&word_lowercase) {
                            token_type = *KEYWORDS.get(&word_lowercase).unwrap();
                        }

                        return Token {
                            token_type,
                            literal: word,
                        };
                    }

                    if character.is_digit(10) {
                        let integer = self.get_integer();

                        return Token {
                            token_type: TokenType::INTEGER,
                            literal: integer,
                        };
                    }

                    return Token {
                        token_type: TokenType::ILLEGAL,
                        literal: character.to_string(),
                    };
                }
            },
            None => {
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
                        break;
                    }
                }
                None => {
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

        return String::from_iter(chars.iter());
    }

    fn get_word(&mut self) -> String {
        let mut chars: Vec<char> = vec![];

        loop {
            match self.current_character {
                Some(character) => {
                    if character.is_alphabetic() {
                        chars.push(character);
                    } else {
                        break;
                    }
                }
                None => {
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

        return String::from_iter(chars.iter());
    }

    fn get_next_character(&mut self) {
        self.increment_character_index();

        match self.current_character {
            Some(character) => match character {
                ' ' | '\t' | '\n' | '\r' => self.get_next_character(),
                _ => {}
            },
            None => {}
        }
    }

    fn increment_character_index(&mut self) {
        self.current_index += 1;
        self.current_character = self.code.chars().nth(self.current_index as usize);
        self.next_character = self.code.chars().nth((self.current_index + 1) as usize);
    }
}

#[cfg(test)]
mod tests;
