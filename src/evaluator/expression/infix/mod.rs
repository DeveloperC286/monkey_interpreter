use crate::evaluator::model::evaluator_context::EvaluatorContext;
use crate::evaluator::model::object::Object;
use crate::lexical_analysis::model::token::Token;
use crate::syntax_analysis::model::abstract_syntax_tree::syntax_tree_node::Expression;

pub fn evaluate(
    evaluator_context: EvaluatorContext,
    left_hand: Expression,
    operator_token: Token,
    right_hand: Expression,
) -> (EvaluatorContext, Object) {
    match crate::evaluator::expression::evaluate(evaluator_context, left_hand) {
        (returned_evaluator_context, Object::INTEGER { value: left_value }) => {
            match crate::evaluator::expression::evaluate(returned_evaluator_context, right_hand) {
                (returned_evaluator_context, Object::INTEGER { value: right_value }) => {
                    match operator_token {
                        Token::PLUS => (
                            returned_evaluator_context,
                            Object::INTEGER {
                                value: left_value + right_value,
                            },
                        ),
                        Token::MINUS => (
                            returned_evaluator_context,
                            Object::INTEGER {
                                value: left_value - right_value,
                            },
                        ),
                        Token::MULTIPLY => (
                            returned_evaluator_context,
                            Object::INTEGER {
                                value: left_value * right_value,
                            },
                        ),
                        Token::DIVIDE => (
                            returned_evaluator_context,
                            Object::INTEGER {
                                value: left_value / right_value,
                            },
                        ),
                        Token::GREATER_THAN => match left_value > right_value {
                            true => (returned_evaluator_context, Object::TRUE),
                            false => (returned_evaluator_context, Object::FALSE),
                        },
                        Token::LESSER_THAN => match left_value < right_value {
                            true => (returned_evaluator_context, Object::TRUE),
                            false => (returned_evaluator_context, Object::FALSE),
                        },
                        Token::EQUALS => match left_value == right_value {
                            true => (returned_evaluator_context, Object::TRUE),
                            false => (returned_evaluator_context, Object::FALSE),
                        },
                        Token::NOT_EQUALS => match left_value != right_value {
                            true => (returned_evaluator_context, Object::TRUE),
                            false => (returned_evaluator_context, Object::FALSE),
                        },
                        _ => panic!("Operator token is not operator token."),
                    }
                }
                (returned_evaluator_context, _) => {
                    (returned_evaluator_context, Object::TYPE_MISMATCH)
                }
            }
        }
        (returned_evaluator_context, Object::TRUE) => {
            match crate::evaluator::expression::evaluate(returned_evaluator_context, right_hand) {
                (returned_evaluator_context, Object::TRUE) => (
                    returned_evaluator_context,
                    evaluate_same_boolean(operator_token),
                ),
                (returned_evaluator_context, Object::FALSE) => (
                    returned_evaluator_context,
                    evaluate_opposite_boolean(operator_token),
                ),
                (returned_evaluator_context, _) => {
                    (returned_evaluator_context, Object::TYPE_MISMATCH)
                }
            }
        }
        (returned_evaluator_context, Object::FALSE) => {
            match crate::evaluator::expression::evaluate(returned_evaluator_context, right_hand) {
                (returned_evaluator_context, Object::FALSE) => (
                    returned_evaluator_context,
                    evaluate_same_boolean(operator_token),
                ),
                (returned_evaluator_context, Object::TRUE) => (
                    returned_evaluator_context,
                    evaluate_opposite_boolean(operator_token),
                ),
                (returned_evaluator_context, _) => {
                    (returned_evaluator_context, Object::TYPE_MISMATCH)
                }
            }
        }
        (returned_evaluator_context, _) => (returned_evaluator_context, Object::NULL),
    }
}

fn evaluate_same_boolean(operator_token: Token) -> Object {
    match operator_token {
        Token::EQUALS => Object::TRUE,
        Token::NOT_EQUALS => Object::FALSE,
        _ => Object::UNKNOWN_OPERATOR,
    }
}

fn evaluate_opposite_boolean(operator_token: Token) -> Object {
    match operator_token {
        Token::EQUALS => Object::FALSE,
        Token::NOT_EQUALS => Object::TRUE,
        _ => Object::UNKNOWN_OPERATOR,
    }
}

#[cfg(test)]
mod tests;
