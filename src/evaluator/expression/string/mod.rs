use crate::evaluator::model::evaluation_error::EvaluationError;
use crate::evaluator::model::object::Object;
use crate::evaluator::Evaluator;

impl Evaluator {
    pub(super) fn evaluate_string(&self, string: String) -> Result<Object, EvaluationError> {
        Ok(Object::String { value: string })
    }
}

#[cfg(test)]
mod tests;
