use crate::evaluator::model::evaluator_context::EvaluatorContext;
use crate::evaluator::model::object::Object;
use crate::syntax_analysis::model::abstract_syntax_tree::syntax_tree_node::*;

mod boolean;
mod if_expression;
mod infix;
mod integer;
mod prefix;

pub fn evaluate(
    evaluator_context: EvaluatorContext,
    expression: Expression,
) -> (EvaluatorContext, Object) {
    match expression {
        Expression::Integer { integer_token } => {
            (evaluator_context, integer::evaluate(integer_token))
        }
        Expression::Boolean { boolean_token } => {
            (evaluator_context, boolean::evaluate(boolean_token))
        }
        Expression::Prefix {
            prefix_token,
            right_hand,
        } => prefix::evaluate(evaluator_context, prefix_token, *right_hand),
        Expression::Infix {
            left_hand,
            operator_token,
            right_hand,
        } => infix::evaluate(evaluator_context, *left_hand, operator_token, *right_hand),
        Expression::If {
            condition,
            consequence,
            alternative,
        } => if_expression::evaluate(evaluator_context, *condition, *consequence, *alternative),
        _ => (evaluator_context, Object::NULL),
    }
}
