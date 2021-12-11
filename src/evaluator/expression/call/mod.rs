use crate::evaluator::model::evaluation_error::EvaluationError;
use crate::evaluator::model::object::Object;
use crate::evaluator::Evaluator;
use crate::syntax_analysis::model::syntax_tree_node::Expression;

impl Evaluator {
    pub(super) fn evaluate_call_expression(
        &mut self,
        function: Expression,
        arguments: Vec<Expression>,
    ) -> Result<Object, EvaluationError> {
        match self.evaluate_expression(function)? {
            Object::Function { parameters, block } => {
                self.environment.push();

                for (argument, parameter_identifier) in arguments.into_iter().zip(parameters) {
                    let argument_evaluation = self.evaluate_expression(argument)?;

                    self.environment
                        .set(parameter_identifier, argument_evaluation);
                }

                let block_call_evaluation = self.evaluate_block(block)?;
                self.environment.pop();
                Ok(block_call_evaluation)
            }
            _ => Err(EvaluationError::UncallableObject),
        }
    }
}

#[cfg(test)]
mod tests;
