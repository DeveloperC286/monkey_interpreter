use crate::evaluator::Evaluator;
use crate::evaluator::Object;
use crate::syntax_analysis::Expression;

impl Evaluator {
    pub(super) fn evaluate_call_expression(
        &mut self,
        function: &Expression,
        arguments: &[Expression],
    ) -> anyhow::Result<Object> {
        match self.evaluate_expression(function)? {
            Object::Function {
                parameters,
                block,
                scope,
            } => {
                if arguments.len() != parameters.len() {
                    anyhow::bail!(
                        "Wrong number of arguments, want={}, got={}.",
                        parameters.len(),
                        arguments.len()
                    );
                }

                // The arguments are evaluated within the calling scope, before entering the
                // function's own, as that is where their identifiers are bound.
                let mut argument_evaluations = Vec::with_capacity(arguments.len());

                for argument in arguments {
                    argument_evaluations.push(self.evaluate_expression(argument)?);
                }

                // The body is evaluated within a fresh scope enclosed by the scope the
                // function was defined within, not the scope calling it.
                let calling_scope = self.environment.push(scope);

                for (argument_evaluation, parameter_identifier) in
                    argument_evaluations.into_iter().zip(parameters)
                {
                    self.environment
                        .set(parameter_identifier, argument_evaluation);
                }

                let block_call_evaluation = self.evaluate_block(&block);
                self.environment.pop(calling_scope);
                block_call_evaluation
            }
            object => anyhow::bail!(
                "Cannot call an object of type {}, only functions are callable.",
                object.type_name()
            ),
        }
    }
}
