use crate::evaluator::model::object::Object;
use crate::evaluator::Evaluator;

impl Evaluator {
    pub(super) fn evaluate_integer(&self, literal: i64) -> anyhow::Result<Object> {
        Ok(Object::Integer { value: literal })
    }
}
