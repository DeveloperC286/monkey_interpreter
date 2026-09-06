use std::fmt;
use std::rc::Rc;

use crate::evaluator::environment::Scope;
use crate::syntax_analysis::Block;

#[derive(Clone, PartialEq)]
pub enum Object {
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
        /// Shared so passing a function around, and looking one up, does not copy its body.
        block: Rc<Block>,
        /// A handle on the scope the function was defined within, captured so the free
        /// variables of its body resolve there rather than wherever the function is called.
        scope: Scope,
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

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Return { value } => f.debug_struct("Return").field("value", value).finish(),
            Object::Integer { value } => f.debug_struct("Integer").field("value", value).finish(),
            Object::String { value } => f.debug_struct("String").field("value", value).finish(),
            Object::True => f.write_str("True"),
            Object::False => f.write_str("False"),
            Object::Null => f.write_str("Null"),
            // The captured scope is deliberately omitted, a function is reachable from the
            // scope it captured, such as `let f = fn() { f() };`, so formatting the scope
            // would recurse forever.
            Object::Function {
                parameters, block, ..
            } => f
                .debug_struct("Function")
                .field("parameters", parameters)
                .field("block", block)
                .finish_non_exhaustive(),
        }
    }
}

impl Object {
    pub(crate) fn type_name(&self) -> &'static str {
        match self {
            Object::Return { .. } => "RETURN",
            Object::Integer { .. } => "INTEGER",
            Object::String { .. } => "STRING",
            Object::True | Object::False => "BOOLEAN",
            Object::Null => "NULL",
            Object::Function { .. } => "FUNCTION",
        }
    }
}
