pub mod syntax_tree_node;
use syntax_tree_node::SyntaxTreeNode;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AbstractSyntaxTree {
    pub program: Vec<SyntaxTreeNode>,
    pub syntax_parsing_errors: Vec<String>,
}
