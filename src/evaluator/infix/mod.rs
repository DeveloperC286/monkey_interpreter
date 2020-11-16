use crate::evaluator::object::Object;
use crate::lexical_analysis::token::Token;
use crate::syntax_analysis::abstract_syntax_tree::syntax_tree_node::Expression;

pub fn evaluate(left_hand: Expression, operator_token: Token, right_hand: Expression) -> Object {
    match crate::evaluator::evaluate_expression(left_hand) {
        Object::INTEGER { value: left_value } => {
            match crate::evaluator::evaluate_expression(right_hand) {
                Object::INTEGER { value: right_value } => match operator_token {
                    Token::PLUS => Object::INTEGER {
                        value: left_value + right_value,
                    },
                    Token::MINUS => Object::INTEGER {
                        value: left_value - right_value,
                    },
                    Token::MULTIPLY => Object::INTEGER {
                        value: left_value * right_value,
                    },
                    Token::DIVIDE => Object::INTEGER {
                        value: left_value / right_value,
                    },
                    Token::GREATER_THAN => match left_value > right_value {
                        true => Object::TRUE,
                        false => Object::FALSE,
                    },
                    Token::LESSER_THAN => match left_value < right_value {
                        true => Object::TRUE,
                        false => Object::FALSE,
                    },
                    Token::EQUALS => match left_value == right_value {
                        true => Object::TRUE,
                        false => Object::FALSE,
                    },
                    Token::NOT_EQUALS => match left_value != right_value {
                        true => Object::TRUE,
                        false => Object::FALSE,
                    },
                    _ => panic!("Operator token is not operator token."),
                },
                _ => Object::TYPE_MISMATCH,
            }
        }
        Object::TRUE => match crate::evaluator::evaluate_expression(right_hand) {
            Object::TRUE => evaluate_same_boolean(operator_token),
            Object::FALSE => evaluate_opposite_boolean(operator_token),
            _ => Object::TYPE_MISMATCH,
        },
        Object::FALSE => match crate::evaluator::evaluate_expression(right_hand) {
            Object::FALSE => evaluate_same_boolean(operator_token),
            Object::TRUE => evaluate_opposite_boolean(operator_token),
            _ => Object::TYPE_MISMATCH,
        },
        _ => Object::NULL,
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
