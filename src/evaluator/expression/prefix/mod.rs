use crate::evaluator::model::evaluation_error::EvaluationError;
use crate::evaluator::model::object::Object;
use crate::evaluator::Evaluator;
use crate::syntax_analysis::model::syntax_tree_node::Expression;

impl Evaluator {
    pub(super) fn evaluate_not_prefix_expression(
        &mut self,
        right_hand_expression: Expression,
    ) -> Result<Object, EvaluationError> {
        let object = self.evaluate_expression(right_hand_expression)?;

        match object {
            Object::True => Ok(Object::False),
            Object::False => Ok(Object::True),
            _ => Err(EvaluationError::TypeMismatch),
        }
    }

    pub(super) fn evaluate_minus_prefix_expression(
        &mut self,
        right_hand_expression: Expression,
    ) -> Result<Object, EvaluationError> {
        let object = self.evaluate_expression(right_hand_expression)?;

        match object {
            Object::Integer { value } => Ok(Object::Integer { value: -value }),
            _ => Err(EvaluationError::TypeMismatch),
        }
    }
}
