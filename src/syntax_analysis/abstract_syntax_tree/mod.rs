pub mod syntax_tree_node;
use syntax_tree_node::SyntaxTreeNode;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AbstractSyntaxTree {
    pub program: Vec<SyntaxTreeNode>,
}
