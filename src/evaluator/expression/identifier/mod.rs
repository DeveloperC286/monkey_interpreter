use crate::evaluator::Evaluator;
use crate::evaluator::Object;

impl Evaluator {
    pub(super) fn evaluate_identifier_expression(
        &self,
        identifier: String,
    ) -> anyhow::Result<Object> {
        match self.environment.get(&identifier) {
            Some(object) => Ok(object),
            None => anyhow::bail!("Identifier not found: {}.", identifier),
        }
    }
}
