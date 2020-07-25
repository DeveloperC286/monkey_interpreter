use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Token {
    ILLEGAL { literal: String },
    EOF,

    // Identifiers
    IDENTIFIER { literal: String },
    INTEGER { literal: String },

    // Operators
    ASSIGN,
    PLUS,
    MINUS,
    DIVIDE,
    MULTIPLY,
    GREATER_THAN,
    LESSER_THAN,
    NOT,
    EQUALS,
    NOT_EQUALS,

    // Delimiters
    COMMA,
    SEMI_COLON,

    // Brackets
    OPENING_ROUND_BRACKET,
    CLOSING_ROUND_BRACKET,

    OPENING_CURLY_BRACKET,
    CLOSING_CURLY_BRACKET,

    //Keywords
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}
