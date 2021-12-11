use crate::syntax_analysis::model::syntax_tree_node::Block;

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
    Function {
        parameters: Vec<String>,
        block: Block,
    },
}
