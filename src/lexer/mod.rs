mod token;
use token::{Token, TokenType};

pub struct Lexer {
    pub code: String,
    current_character: char,
    current_index: i32,
}

impl Lexer {
    pub fn new(code: String) -> Lexer {
        return Lexer {
            code: code,
            current_character: '0',
            current_index: -1,
        };
    }

    pub fn get_next_token(&mut self) -> Token {
        self.increment_character();

        match self.current_character {
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
                    token_type: TokenType::EOF,
                    literal: "".to_string(),
                };
            }
        }
    }

    fn increment_character(&mut self) {
        self.current_index += 1;
        let character_option: Option<char> = self.code.chars().nth(self.current_index as usize);

        match character_option {
            Some(character) => self.current_character = character,
            None => self.current_character = '0',
        }
    }
}

#[cfg(test)]
mod tests;
