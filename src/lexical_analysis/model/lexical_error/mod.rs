use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum LexicalError {
    #[error("Illegal character provided for lexical analysis {0:?}.")]
    IllegalCharacter(char),
}
