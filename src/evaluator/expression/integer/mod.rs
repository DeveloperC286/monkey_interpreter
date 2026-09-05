use crate::evaluator::Evaluator;
use crate::evaluator::Object;

impl Evaluator {
    pub(super) fn evaluate_integer(&self, literal: i64) -> anyhow::Result<Object> {
        Ok(Object::Integer { value: literal })
    }
}
