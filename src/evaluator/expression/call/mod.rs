use crate::evaluator::Evaluator;
use crate::evaluator::Object;
use crate::syntax_analysis::Block;
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
                // The call's environment is popped before the result is propagated, so an error
                // inside the call does not leave an orphaned environment behind.
                let call_evaluation = self.evaluate_call(arguments, parameters, block);
                self.environment.pop();
                call_evaluation
            }
            object => anyhow::bail!(
                "Cannot call an object of type {}, only functions are callable.",
                object.type_name()
            ),
        }
    }

    fn evaluate_call(
        &mut self,
        arguments: Vec<Expression>,
        parameters: Vec<String>,
        block: Block,
    ) -> anyhow::Result<Object> {
        for (argument, parameter_identifier) in arguments.into_iter().zip(parameters) {
            let argument_evaluation = self.evaluate_expression(argument)?;

            self.environment
                .set(parameter_identifier, argument_evaluation);
        }

        self.evaluate_block(block)
    }
}
