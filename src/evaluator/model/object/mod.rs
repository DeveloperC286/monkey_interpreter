use crate::syntax_analysis::model::abstract_syntax_tree::syntax_tree_node::Block;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Object {
    Return {
        value: Box<Object>,
    },
    Integer {
        value: i32,
    },
    True,
    False,
    Null,
    Error {
        error_type: ErrorType,
    },
    Function {
        parameters: Vec<String>,
        block: Block,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ErrorType {
    TypeMismatch,
    UnknownOperator,
    UnassignableObject,
    UncallableObject,
}
