use crate::evaluator::Object;
use crate::evaluator::Evaluator;
use crate::syntax_analysis::{Block, Expression};

impl Evaluator {
    pub(super) fn evaluate_if_expression(
        &mut self,
        condition: Expression,
        consequence: Block,
        alternative: Option<Block>,
    ) -> anyhow::Result<Object> {
        match self.evaluate_expression(condition)? {
            Object::Null | Object::False => match alternative {
                Some(block) => self.evaluate_block(block),
                None => Ok(Object::Null),
            },
            _ => self.evaluate_block(consequence),
        }
    }
}
