use crate::evaluator::Evaluator;
use crate::evaluator::model::object::Object;
use crate::syntax_analysis::model::syntax_tree_node::Expression;

impl Evaluator {
    pub(super) fn evaluate_not_prefix_expression(
        &mut self,
        right_hand_expression: Expression,
    ) -> anyhow::Result<Object> {
        let object = self.evaluate_expression(right_hand_expression)?;

        match object {
            Object::True => Ok(Object::False),
            Object::False => Ok(Object::True),
            object => anyhow::bail!(
                "Type mismatch, cannot apply the ! prefix operator to an operand of type {}.",
                object.type_name()
            ),
        }
    }

    pub(super) fn evaluate_minus_prefix_expression(
        &mut self,
        right_hand_expression: Expression,
    ) -> anyhow::Result<Object> {
        let object = self.evaluate_expression(right_hand_expression)?;

        match object {
            Object::Integer { value } => Ok(Object::Integer { value: -value }),
            object => anyhow::bail!(
                "Type mismatch, cannot apply the - prefix operator to an operand of type {}.",
                object.type_name()
            ),
        }
    }
}
