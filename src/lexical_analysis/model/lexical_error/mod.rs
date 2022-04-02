use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum LexicalError {
    #[error("Unparsable context for lexical analysis {0:?}.")]
    UnparsableContext(String),
    #[error("String not closed before the end of the code.")]
    StringNotClosed,
    #[error("Illegal escaping of the character {0:?}.")]
    IllegalEscaping(char),
}
