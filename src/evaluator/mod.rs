use crate::evaluator::model::object::Object;

use crate::syntax_analysis::model::abstract_syntax_tree::syntax_tree_node::*;
use crate::syntax_analysis::model::abstract_syntax_tree::AbstractSyntaxTree;

pub(crate) mod model;

#[cfg(test)]
#[macro_use]
mod tests;

mod expression;
mod statement;

pub(crate) struct Evaluator {}

impl Evaluator {
    pub(crate) fn new() -> Evaluator {
        Evaluator {}
    }

    pub(crate) fn evaluate(&self, abstract_syntax_tree: AbstractSyntaxTree) -> Object {
        if !abstract_syntax_tree.syntax_parsing_errors.is_empty() {
            panic!("Syntax errors unable to evaluate.");
        }

        let mut object = Object::Null;

        for syntax_tree_node in abstract_syntax_tree.abstract_syntax_tree {
            object = self.evaluate_node(syntax_tree_node);

            match object.clone() {
                Object::Return { value } => {
                    object = *value;
                    break;
                }
                Object::Error { error_type: _ } => break,
                _ => {}
            }
        }

        object
    }

    fn evaluate_block(&self, block: Block) -> Object {
        let mut object = Object::Null;

        for syntax_tree_node in block.nodes {
            object = self.evaluate_node(syntax_tree_node);

            match object.clone() {
                Object::Return { value: _ } | Object::Error { error_type: _ } => break,
                _ => {}
            }
        }

        object
    }

    fn evaluate_node(&self, syntax_tree_node: SyntaxTreeNode) -> Object {
        match syntax_tree_node {
            SyntaxTreeNode::Expression { expression } => self.evaluate_expression(expression),
            SyntaxTreeNode::Statement { statement } => self.evaluate_statement(statement),
        }
    }
}
