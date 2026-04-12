use crate::evaluator::Object;
use crate::evaluator::Evaluator;
use crate::syntax_analysis::*;

impl Evaluator {
    pub(super) fn evaluate_return_statement(
        &mut self,
        expression: Expression,
    ) -> anyhow::Result<Object> {
        Ok(Object::Return {
            value: Box::new(self.evaluate_expression(expression)?),
        })
    }
}
