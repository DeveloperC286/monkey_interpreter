use crate::evaluator::model::evaluation_error::EvaluationError;
use crate::evaluator::model::object::Object;
use crate::evaluator::Evaluator;

impl Evaluator {
    pub(super) fn evaluate_boolean(&self, literal: bool) -> Result<Object, EvaluationError> {
        match literal {
            true => Ok(Object::True),
            false => Ok(Object::False),
        }
    }
}
