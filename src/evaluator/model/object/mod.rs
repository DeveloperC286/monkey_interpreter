use std::fmt;

use crate::syntax_analysis::model::syntax_tree_node::Block;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Object {
    Return {
        value: Box<Object>,
    },
    Integer {
        value: i64,
    },
    String {
        value: String,
    },
    True,
    False,
    Null,
    Function {
        parameters: Vec<String>,
        block: Block,
    },
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Return { value } => write!(f, "{value}"),
            Object::Integer { value } => write!(f, "{value}"),
            Object::String { value } => write!(f, "{value}"),
            Object::True => write!(f, "true"),
            Object::False => write!(f, "false"),
            Object::Null => write!(f, "null"),
            Object::Function { parameters, .. } => {
                write!(f, "fn({}) {{ ... }}", parameters.join(", "))
            }
        }
    }
}
