use crate::evaluator::model::object::{ErrorType, Object};
use crate::evaluator::Evaluator;
use crate::syntax_analysis::model::syntax_tree_node::Expression;

impl Evaluator {
    pub(super) fn evaluate_call_expression(
        &mut self,
        function: Expression,
        arguments: Vec<Expression>,
    ) -> Object {
        match self.evaluate_expression(function) {
            Object::Function { parameters, block } => {
                self.environment.push();

                for (argument, parameter_identifier) in arguments.into_iter().zip(parameters) {
                    let argument_evaluation = self.evaluate_expression(argument);

                    if let Object::Error { error_type } = argument_evaluation.clone() {
                        self.environment.pop();
                        return Object::Error { error_type };
                    }

                    self.environment
                        .set(parameter_identifier, argument_evaluation);
                }

                let block_call_evaluation = self.evaluate_block(block);
                self.environment.pop();
                block_call_evaluation
            }
            Object::Error { error_type } => Object::Error { error_type },
            _ => Object::Error {
                error_type: ErrorType::UncallableObject,
            },
        }
    }
}

#[cfg(test)]
mod tests;
