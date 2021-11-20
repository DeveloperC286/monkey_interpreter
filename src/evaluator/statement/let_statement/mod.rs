use crate::evaluator::model::object::{ErrorType, Object};
use crate::evaluator::Evaluator;
use crate::syntax_analysis::model::abstract_syntax_tree::syntax_tree_node::*;

impl Evaluator {
    pub(super) fn evaluate_let_statement(
        &mut self,
        identifier: String,
        expression: Expression,
    ) -> Object {
        let expression = self.evaluate_expression(expression);

        match expression.clone() {
            Object::Return { value: _ } => {
                return Object::Error {
                    error_type: ErrorType::UnassignableObject,
                };
            }
            Object::Error { error_type } => {
                return Object::Error { error_type };
            }
            _ => {}
        }

        self.variables.insert(identifier, expression);

        Object::Null
    }
}

#[cfg(test)]
mod tests;
