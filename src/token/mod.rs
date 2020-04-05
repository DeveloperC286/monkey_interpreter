#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    // Identifiers
    IDENTIFIER,
    INTEGER,

    // Operators
    ASSIGN,
    PLUS,
    MINUS,
    DIVIDE,
    MULTIPLY,
    GREATER_THAN,
    LESSER_THAN,

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
}
