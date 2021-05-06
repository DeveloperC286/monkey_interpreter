use crate::evaluator::model::evaluator_context::EvaluatorContext;
use crate::evaluator::model::object::Object;
use crate::syntax_analysis::model::abstract_syntax_tree::syntax_tree_node::*;

pub fn evaluate(
    evaluator_context: EvaluatorContext,
    expression: Expression,
) -> (EvaluatorContext, Object) {
    let (returned_evaluator_context, value) =
        crate::evaluator::expression::evaluate(evaluator_context, expression);

    (
        returned_evaluator_context,
        Object::Return {
            value: Box::new(value),
        },
    )
}

#[cfg(test)]
mod tests;
