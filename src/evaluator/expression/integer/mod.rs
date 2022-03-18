use crate::evaluator::model::evaluation_error::EvaluationError;
use crate::evaluator::model::object::Object;
use crate::evaluator::Evaluator;

impl Evaluator {
    pub(super) fn evaluate_integer(&self, literal: i64) -> Result<Object, EvaluationError> {
        Ok(Object::Integer { value: literal })
    }
}
