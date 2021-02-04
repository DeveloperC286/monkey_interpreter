use crate::evaluator::model::evaluator_context::EvaluatorContext;
use crate::evaluator::model::object::Object;
use crate::syntax_analysis::model::abstract_syntax_tree::syntax_tree_node::{Block, Expression};

pub fn evaluate(
    evaluator_context: EvaluatorContext,
    condition: Expression,
    consequence: Block,
    alternative: Option<Block>,
) -> (EvaluatorContext, Object) {
    match crate::evaluator::expression::evaluate(evaluator_context, condition) {
        (evaluator_context, Object::NULL) | (evaluator_context, Object::FALSE) => match alternative
        {
            Some(block) => crate::evaluator::evaluate_block(evaluator_context, block),
            None => (evaluator_context, Object::NULL),
        },
        (evaluator_context, _) => crate::evaluator::evaluate_block(evaluator_context, consequence),
    }
}

#[cfg(test)]
mod tests;
