use crate::syntax_analysis::Block;

#[derive(Debug, Clone, PartialEq)]
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
        block: Block,
    },
}
