use crate::evaluator::Evaluator;
use crate::evaluator::Object;

impl Evaluator {
    pub(super) fn evaluate_string(&self, string: &str) -> anyhow::Result<Object> {
        Ok(Object::String {
            value: string.to_owned(),
        })
    }
}
