use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Object {
    RETURN { value: Box<Object> },
    INTEGER { value: i32 },
    TRUE,
    FALSE,
    NULL,
    TYPE_MISMATCH,
    UNKNOWN_OPERATOR,
}
