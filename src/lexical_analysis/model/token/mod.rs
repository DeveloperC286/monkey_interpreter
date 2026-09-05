use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Position {
    pub(crate) line: usize,
    pub(crate) column: usize,
}

impl fmt::Display for Position {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "line {}, column {}", self.line, self.column)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct PositionedToken {
    pub(crate) token: Token,
    pub(crate) position: Position,
}

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

    //Keywords
    Function,
    Let,
    If,
    Else,
    Return,
}
