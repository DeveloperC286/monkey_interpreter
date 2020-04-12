pub mod statement_node;
pub mod syntax_tree_node;

use syntax_tree_node::SyntaxTreeNode;

pub struct AbstractSyntaxTree {
    pub program: Vec<Box<dyn SyntaxTreeNode>>,
}
