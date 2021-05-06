#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Return { value: Box<Object> },
    Integer { value: i32 },
    True,
    False,
    Null,
    TypeMismatch,
    UnknownOperator,
}
