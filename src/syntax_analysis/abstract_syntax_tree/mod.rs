use crate::syntax_analysis::abstract_syntax_tree::syntax_tree_node::SyntaxTreeNode;

pub mod syntax_tree_node;

#[derive(Debug)]
pub struct AbstractSyntaxTree {
    pub abstract_syntax_tree: Vec<SyntaxTreeNode>,
    pub syntax_parsing_errors: Vec<String>,
}
