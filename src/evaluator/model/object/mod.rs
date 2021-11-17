#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Object {
    Return { value: Box<Object> },
    Integer { value: i32 },
    True,
    False,
    Null,
    TypeMismatch,
    UnknownOperator,
}
