use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum EvaluationError {
    #[error("TypeMismatch")]
    TypeMismatch,
    #[error("UnknownOperator")]
    UnknownOperator,
    #[error("UnassignableObject")]
    UnassignableObject,
    #[error("UncallableObject")]
    UncallableObject,
    #[error("NotBooleanToken")]
    NotBooleanToken,
    #[error("NotIntegerToken")]
    NotIntegerToken,
    #[error("NotPrefixToken")]
    NotPrefixToken,
}
