use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexicalError {
    #[error("Illegal character provided for lexical analysis {0:?}.")]
    IllegalCharacter(char),
}
