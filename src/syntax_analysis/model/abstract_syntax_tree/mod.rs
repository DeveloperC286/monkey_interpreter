use crate::syntax_analysis::model::abstract_syntax_tree::syntax_tree_node::SyntaxTreeNode;

pub(crate) mod syntax_tree_node;

#[derive(Debug)]
pub(crate) struct AbstractSyntaxTree {
    pub(crate) abstract_syntax_tree: Vec<SyntaxTreeNode>,
    pub(crate) syntax_parsing_errors: Vec<String>,
}
