use crate::evaluator::Object;
use crate::evaluator::Evaluator;
use crate::syntax_analysis::Expression;

impl Evaluator {
    pub(super) fn evaluate_not_prefix_expression(
        &mut self,
        right_hand_expression: Expression,
    ) -> anyhow::Result<Object> {
        let object = self.evaluate_expression(right_hand_expression)?;

        match object {
            Object::True => Ok(Object::False),
            Object::False => Ok(Object::True),
            _ => anyhow::bail!("TypeMismatch"),
        }
    }

    pub(super) fn evaluate_minus_prefix_expression(
        &mut self,
        right_hand_expression: Expression,
    ) -> anyhow::Result<Object> {
        let object = self.evaluate_expression(right_hand_expression)?;

        match object {
            Object::Integer { value } => Ok(Object::Integer { value: -value }),
            _ => anyhow::bail!("TypeMismatch"),
        }
    }
}
