use crate::evaluator::model::environment::Environment;
use crate::evaluator::model::object::Object;
use crate::syntax_analysis::model::syntax_tree_node::*;

mod model;

#[cfg(test)]
#[macro_use]
mod tests;

mod expression;
mod statement;

pub(crate) struct Evaluator {
    environment: Environment,
}

impl Evaluator {
    pub(crate) fn new() -> Evaluator {
        Evaluator {
            environment: Environment::new(),
        }
    }

    pub(crate) fn evaluate(&mut self, abstract_syntax_tree: Vec<SyntaxTreeNode>) -> Object {
        let mut object = Object::Null;

        for syntax_tree_node in abstract_syntax_tree {
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

    fn evaluate_block(&mut self, block: Block) -> Object {
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

    fn evaluate_node(&mut self, syntax_tree_node: SyntaxTreeNode) -> Object {
        match syntax_tree_node {
            SyntaxTreeNode::Expression { expression } => self.evaluate_expression(expression),
            SyntaxTreeNode::Statement { statement } => self.evaluate_statement(statement),
        }
    }
}
