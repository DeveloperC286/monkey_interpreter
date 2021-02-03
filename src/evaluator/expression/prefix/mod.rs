use crate::evaluator::evaluator_context::EvaluatorContext;
use crate::evaluator::object::Object;
use crate::lexical_analysis::model::token::Token;
use crate::syntax_analysis::abstract_syntax_tree::syntax_tree_node::Expression;

pub fn evaluate(
    evaluator_context: EvaluatorContext,
    prefix_token: Token,
    right_hand_expression: Expression,
) -> (EvaluatorContext, Object) {
    let (returned_evaluator_context, object) =
        crate::evaluator::expression::evaluate(evaluator_context, right_hand_expression);
    match prefix_token {
        Token::NOT => match object {
            Object::TRUE => (returned_evaluator_context, Object::FALSE),
            Object::FALSE => (returned_evaluator_context, Object::TRUE),
            _ => (returned_evaluator_context, Object::UNKNOWN_OPERATOR),
        },
        Token::MINUS => match object {
            Object::INTEGER { value } => (
                returned_evaluator_context,
                Object::INTEGER { value: -value },
            ),
            _ => (returned_evaluator_context, Object::UNKNOWN_OPERATOR),
        },
        _ => panic!("Prefix token not a prefix token."),
    }
}

#[cfg(test)]
mod tests;
