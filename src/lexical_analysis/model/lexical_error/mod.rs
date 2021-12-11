use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum LexicalError {
    #[error("Illegal character provided for lexical analysis {0:?}.")]
    IllegalCharacter(char),
    #[error("Non numeric characters are within the integer {0:?}.")]
    NonNumericIntegerString(String),
    #[error("String not closed before the end of the code.")]
    StringNotClosed,
}
