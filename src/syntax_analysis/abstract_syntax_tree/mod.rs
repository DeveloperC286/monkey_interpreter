pub mod syntax_tree_node;

use syntax_tree_node::SyntaxTreeNode;

pub struct AbstractSyntaxTree {
    pub program: Vec<SyntaxTreeNode>,
}
