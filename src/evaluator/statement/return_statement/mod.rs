use crate::evaluator::model::object::Object;
use crate::evaluator::Evaluator;
use crate::syntax_analysis::model::syntax_tree_node::*;

impl Evaluator {
    pub(super) fn evaluate_return_statement(&mut self, expression: Expression) -> Object {
        Object::Return {
            value: Box::new(self.evaluate_expression(expression)),
        }
    }
}

#[cfg(test)]
mod tests;
