#[derive(PartialEq, PartialOrd)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

#[derive(PartialEq, PartialOrd)]
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

    // Delimiters
    COMMA,
    SEMI_COLON,

    // Brackets
    OPENING_ROUND_BRACKET,
    CLOSING_ROUND_BRACKET,

    OPENING_CURLY_BRACKET,
    CLOSING_CURLY_BRACKET,

    OPENING_SQUARE_BRACKET,
    CLOSING_SQUARE_BRACKET,

    OPENING_ANGLE_BRACKET,
    CLOSING_ANGLE_BRACKET,

    //Keywords
    FUNCTION,
    LET,
}
