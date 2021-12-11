#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Token {
    EndOfFile,

    // Identifiers
    Identifier { literal: String },
    Integer { literal: String },

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
    True,
    False,
    If,
    Else,
    Return,
}
