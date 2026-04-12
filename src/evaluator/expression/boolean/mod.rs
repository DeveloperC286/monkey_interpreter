use crate::evaluator::Object;
use crate::evaluator::Evaluator;

impl Evaluator {
    pub(super) fn evaluate_boolean(&self, literal: bool) -> anyhow::Result<Object> {
        match literal {
            true => Ok(Object::True),
            false => Ok(Object::False),
        }
    }
}
