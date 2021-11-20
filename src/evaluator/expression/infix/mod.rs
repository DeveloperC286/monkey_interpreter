use crate::evaluator::model::object::{ErrorType, Object};
use crate::evaluator::Evaluator;
use crate::lexical_analysis::model::token::Token;
use crate::syntax_analysis::model::abstract_syntax_tree::syntax_tree_node::Expression;

impl Evaluator {
    pub(super) fn evaluate_infix_expression(
        &mut self,
        left_hand: Expression,
        operator_token: Token,
        right_hand: Expression,
    ) -> Object {
        match self.evaluate_expression(left_hand) {
            Object::Integer { value: left_value } => match self.evaluate_expression(right_hand) {
                Object::Integer { value: right_value } => match operator_token {
                    Token::Plus => Object::Integer {
                        value: left_value + right_value,
                    },
                    Token::Minus => Object::Integer {
                        value: left_value - right_value,
                    },
                    Token::Multiply => Object::Integer {
                        value: left_value * right_value,
                    },
                    Token::Divide => Object::Integer {
                        value: left_value / right_value,
                    },
                    Token::GreaterThan => match left_value > right_value {
                        true => Object::True,
                        false => Object::False,
                    },
                    Token::LesserThan => match left_value < right_value {
                        true => Object::True,
                        false => Object::False,
                    },
                    Token::Equals => match left_value == right_value {
                        true => Object::True,
                        false => Object::False,
                    },
                    Token::NotEquals => match left_value != right_value {
                        true => Object::True,
                        false => Object::False,
                    },
                    _ => panic!("Operator token is not operator token."),
                },
                _ => Object::Error {
                    error_type: ErrorType::TypeMismatch,
                },
            },
            Object::True => match self.evaluate_expression(right_hand) {
                Object::True => evaluate_same_boolean(operator_token),
                Object::False => evaluate_opposite_boolean(operator_token),
                _ => Object::Error {
                    error_type: ErrorType::TypeMismatch,
                },
            },
            Object::False => match self.evaluate_expression(right_hand) {
                Object::False => evaluate_same_boolean(operator_token),
                Object::True => evaluate_opposite_boolean(operator_token),
                _ => Object::Error {
                    error_type: ErrorType::TypeMismatch,
                },
            },
            _ => Object::Null,
        }
    }
}

fn evaluate_same_boolean(operator_token: Token) -> Object {
    match operator_token {
        Token::Equals => Object::True,
        Token::NotEquals => Object::False,
        _ => Object::Error {
            error_type: ErrorType::UnknownOperator,
        },
    }
}

fn evaluate_opposite_boolean(operator_token: Token) -> Object {
    match operator_token {
        Token::Equals => Object::False,
        Token::NotEquals => Object::True,
        _ => Object::Error {
            error_type: ErrorType::UnknownOperator,
        },
    }
}

#[cfg(test)]
mod tests;
