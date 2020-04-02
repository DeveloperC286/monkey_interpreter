mod token;
use token::{Token, TokenType};

pub struct Lexer {
    pub code: String,
    current_character: Option<char>,
    current_index: i32,
}

impl Lexer {
    pub fn new(code: String) -> Lexer {
        return Lexer {
            code: code,
            current_character: None,
            current_index: -1,
        };
    }

    pub fn get_next_token(&mut self) -> Token {
        self.get_next_character();

        match self.current_character {
            Some(character) => match character {
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
    }
}

#[cfg(test)]
mod tests;
