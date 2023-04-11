#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Token {
    // Identifiers
    Identifier { literal: String },

    //Values
    Integer { literal: i64 },
    True,
    False,
    String { literal: String },

    // Operators
    Assign,
    Plus,
    Minus,
    Divide,
    Multiply,
    GreaterThan,
    LesserThan,
    Not,
    Equals,
    NotEquals,

    // Delimiters
    Comma,
    SemiColon,

    // Brackets
    OpeningRoundBracket,
    ClosingRoundBracket,

    OpeningCurlyBracket,
    ClosingCurlyBracket,

    OpeningSquareBracket,
    ClosingSquareBracket,

    //Keywords
    Function,
    Let,
    If,
    Else,
    Return,
}
