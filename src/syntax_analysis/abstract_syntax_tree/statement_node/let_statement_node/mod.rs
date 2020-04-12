use super::super::super::super::lexical_analysis::token::Token;
use super::super::syntax_tree_node::SyntaxTreeNode;
use super::StatementNode;

pub struct LetStatementNode {
    pub letToken: Box<Token>,
    pub identifierToken: Box<Token>,
}

impl StatementNode for LetStatementNode {}
impl SyntaxTreeNode for LetStatementNode {}
