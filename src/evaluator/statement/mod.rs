use crate::evaluator::evaluator_context::EvaluatorContext;
use crate::evaluator::object::Object;
use crate::syntax_analysis::abstract_syntax_tree::syntax_tree_node::*;

mod return_statement;

pub fn evaluate(
    evaluator_context: EvaluatorContext,
    statement: Statement,
) -> (EvaluatorContext, Object) {
    match statement {
        Statement::RETURN { expression } => {
            crate::evaluator::statement::return_statement::evaluate(evaluator_context, *expression)
        }
        _ => (evaluator_context, Object::NULL),
    }
}
