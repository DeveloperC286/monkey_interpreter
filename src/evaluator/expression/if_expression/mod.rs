use crate::evaluator::model::object::Object;
use crate::evaluator::Evaluator;
use crate::syntax_analysis::model::abstract_syntax_tree::syntax_tree_node::{Block, Expression};

impl Evaluator {
    pub(super) fn evaluate_if_expression(
        &mut self,
        condition: Expression,
        consequence: Block,
        alternative: Option<Block>,
    ) -> Object {
        match self.evaluate_expression(condition) {
            Object::Null | Object::False => match alternative {
                Some(block) => self.evaluate_block(block),
                None => Object::Null,
            },
            _ => self.evaluate_block(consequence),
        }
    }
}

#[cfg(test)]
mod tests;
