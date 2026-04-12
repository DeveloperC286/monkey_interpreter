use crate::evaluator::Object;
use crate::evaluator::Evaluator;
use crate::syntax_analysis::*;

impl Evaluator {
    pub(super) fn evaluate_let_statement(
        &mut self,
        identifier: String,
        expression: Expression,
    ) -> anyhow::Result<Object> {
        let expression = self.evaluate_expression(expression)?;

        if let Object::Return { value: _ } = expression {
            anyhow::bail!("UnassignableObject");
        }

        self.environment.set(identifier, expression);

        Ok(Object::Null)
    }
}
