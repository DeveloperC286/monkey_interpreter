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
        (returned_evaluator_context, Object::Integer { value: left_value }) => {
            match crate::evaluator::expression::evaluate(returned_evaluator_context, right_hand) {
                (returned_evaluator_context, Object::Integer { value: right_value }) => {
                    match operator_token {
                        Token::Plus => (
                            returned_evaluator_context,
                            Object::Integer {
                                value: left_value + right_value,
                            },
                        ),
                        Token::Minus => (
                            returned_evaluator_context,
                            Object::Integer {
                                value: left_value - right_value,
                            },
                        ),
                        Token::Multiply => (
                            returned_evaluator_context,
                            Object::Integer {
                                value: left_value * right_value,
                            },
                        ),
                        Token::Divide => (
                            returned_evaluator_context,
                            Object::Integer {
                                value: left_value / right_value,
                            },
                        ),
                        Token::GreaterThan => match left_value > right_value {
                            true => (returned_evaluator_context, Object::True),
                            false => (returned_evaluator_context, Object::False),
                        },
                        Token::LesserThan => match left_value < right_value {
                            true => (returned_evaluator_context, Object::True),
                            false => (returned_evaluator_context, Object::False),
                        },
                        Token::Equals => match left_value == right_value {
                            true => (returned_evaluator_context, Object::True),
                            false => (returned_evaluator_context, Object::False),
                        },
                        Token::NotEquals => match left_value != right_value {
                            true => (returned_evaluator_context, Object::True),
                            false => (returned_evaluator_context, Object::False),
                        },
                        _ => panic!("Operator token is not operator token."),
                    }
                }
                (returned_evaluator_context, _) => {
                    (returned_evaluator_context, Object::TypeMismatch)
                }
            }
        }
        (returned_evaluator_context, Object::True) => {
            match crate::evaluator::expression::evaluate(returned_evaluator_context, right_hand) {
                (returned_evaluator_context, Object::True) => (
                    returned_evaluator_context,
                    evaluate_same_boolean(operator_token),
                ),
                (returned_evaluator_context, Object::False) => (
                    returned_evaluator_context,
                    evaluate_opposite_boolean(operator_token),
                ),
                (returned_evaluator_context, _) => {
                    (returned_evaluator_context, Object::TypeMismatch)
                }
            }
        }
        (returned_evaluator_context, Object::False) => {
            match crate::evaluator::expression::evaluate(returned_evaluator_context, right_hand) {
                (returned_evaluator_context, Object::False) => (
                    returned_evaluator_context,
                    evaluate_same_boolean(operator_token),
                ),
                (returned_evaluator_context, Object::True) => (
                    returned_evaluator_context,
                    evaluate_opposite_boolean(operator_token),
                ),
                (returned_evaluator_context, _) => {
                    (returned_evaluator_context, Object::TypeMismatch)
                }
            }
        }
        (returned_evaluator_context, _) => (returned_evaluator_context, Object::Null),
    }
}

fn evaluate_same_boolean(operator_token: Token) -> Object {
    match operator_token {
        Token::Equals => Object::True,
        Token::NotEquals => Object::False,
        _ => Object::UnknownOperator,
    }
}

fn evaluate_opposite_boolean(operator_token: Token) -> Object {
    match operator_token {
        Token::Equals => Object::False,
        Token::NotEquals => Object::True,
        _ => Object::UnknownOperator,
    }
}

#[cfg(test)]
mod tests;
