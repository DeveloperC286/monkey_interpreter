#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Token {
    EndOfFile,

    // Identifiers
    Identifier { literal: String },

    //Values
    Integer { literal: i64 },
    True,
    False,

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

    //Keywords
    Function,
    Let,
    If,
    Else,
    Return,
}
