use crate::evaluator::model::evaluator_context::EvaluatorContext;
use crate::evaluator::model::object::Object;
use crate::lexical_analysis::model::token::Token;
use crate::syntax_analysis::model::abstract_syntax_tree::syntax_tree_node::Expression;

pub(crate) fn evaluate(
    evaluator_context: EvaluatorContext,
    prefix_token: Token,
    right_hand_expression: Expression,
) -> (EvaluatorContext, Object) {
    let (returned_evaluator_context, object) =
        crate::evaluator::expression::evaluate(evaluator_context, right_hand_expression);
    match prefix_token {
        Token::Not => match object {
            Object::True => (returned_evaluator_context, Object::False),
            Object::False => (returned_evaluator_context, Object::True),
            _ => (returned_evaluator_context, Object::UnknownOperator),
        },
        Token::Minus => match object {
            Object::Integer { value } => (
                returned_evaluator_context,
                Object::Integer { value: -value },
            ),
            _ => (returned_evaluator_context, Object::UnknownOperator),
        },
        _ => panic!("Prefix token not a prefix token."),
    }
}

#[cfg(test)]
mod tests;
