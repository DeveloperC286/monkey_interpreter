use crate::evaluator::Evaluator;
use crate::evaluator::Object;
use crate::syntax_analysis::Expression;

impl Evaluator {
    pub(super) fn evaluate_call_expression(
        &mut self,
        function: Expression,
        arguments: Vec<Expression>,
    ) -> anyhow::Result<Object> {
        match self.evaluate_expression(function)? {
            Object::Function { parameters, block } => {
                if arguments.len() != parameters.len() {
                    anyhow::bail!(
                        "Wrong number of arguments, want={}, got={}.",
                        parameters.len(),
                        arguments.len()
                    );
                }

                self.environment.push();

                for (argument, parameter_identifier) in arguments.into_iter().zip(parameters) {
                    let argument_evaluation = self.evaluate_expression(argument)?;

                    self.environment
                        .set(parameter_identifier, argument_evaluation);
                }

                let block_call_evaluation = self.evaluate_block(block)?;
                self.environment.pop();

                // A return only unwinds as far as the function it is in, unwrap
                // it here so it does not leak into the caller as a value.
                Ok(match block_call_evaluation {
                    Object::Return { value } => *value,
                    object => object,
                })
            }
            object => anyhow::bail!(
                "Cannot call an object of type {}, only functions are callable.",
                object.type_name()
            ),
        }
    }
}
