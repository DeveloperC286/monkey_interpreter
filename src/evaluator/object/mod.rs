use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Object {
    INTEGER { value: i32 },
    TRUE,
    FALSE,
    NULL,
}
