use crate::syntax_analysis::*;

mod model;

pub use model::object::Object;
use model::environment::Environment;

mod expression;
mod statement;

#[derive(Debug)]
pub struct Evaluator {
    environment: Environment,
}

impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator {
            environment: Environment::new(),
        }
    }

    pub fn evaluate(
        &mut self,
        abstract_syntax_tree: Vec<SyntaxTreeNode>,
    ) -> anyhow::Result<Object> {
        let mut object = Object::Null;

        for syntax_tree_node in abstract_syntax_tree {
            match self.evaluate_node(syntax_tree_node)? {
                Object::Return { value } => {
                    return Ok(*value);
                }
                value => object = value,
            }
        }

        Ok(object)
    }

    fn evaluate_block(&mut self, block: Block) -> anyhow::Result<Object> {
        let mut object = Object::Null;

        for syntax_tree_node in block.nodes {
            object = self.evaluate_node(syntax_tree_node)?;

            if let Object::Return { value: _ } = &object {
                return Ok(object);
            }
        }

        Ok(object)
    }

    fn evaluate_node(&mut self, syntax_tree_node: SyntaxTreeNode) -> anyhow::Result<Object> {
        match syntax_tree_node {
            SyntaxTreeNode::Expression { expression } => self.evaluate_expression(expression),
            SyntaxTreeNode::Statement { statement } => self.evaluate_statement(statement),
        }
    }
}
