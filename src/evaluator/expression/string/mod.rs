use crate::evaluator::model::object::Object;
use crate::evaluator::Evaluator;

impl Evaluator {
    pub(super) fn evaluate_string(&self, string: String) -> anyhow::Result<Object> {
        Ok(Object::String { value: string })
    }
}
