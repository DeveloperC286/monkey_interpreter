use crate::evaluator::object::Object;
use crate::lexical_analysis::token::Token;
use crate::syntax_analysis::abstract_syntax_tree::syntax_tree_node::*;

pub fn parse_infix(
    left_hand_node: SyntaxTreeNode,
    operator_token: Token,
    right_hand_node: SyntaxTreeNode,
) -> Object {
    return match crate::evaluator::evaluate_node(left_hand_node) {
        Object::INTEGER { value } => {
            let left_value = value;
            match crate::evaluator::evaluate_node(right_hand_node) {
                Object::INTEGER { value } => {
                    let right_value = value;
                    match operator_token {
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
                        }
                        _ => Object::NULL,
                    }
                }
                _ => Object::NULL,
            }
        }
        Object::TRUE => {
            match crate::evaluator::evaluate_node(right_hand_node) {
                Object::TRUE => evaluate_same_boolean(operator_token),
                Object::FALSE => evaluate_opposite_boolean(operator_token),
                _ => Object::NULL,
            }
        }
        Object::FALSE => {
            match crate::evaluator::evaluate_node(right_hand_node) {
                Object::FALSE => evaluate_same_boolean(operator_token),
                Object::TRUE => evaluate_opposite_boolean(operator_token),
                _ => Object::NULL,
            }
        }
        _ => Object::NULL,
    };
}

fn evaluate_same_boolean(operator_token: Token) -> Object {
    match operator_token {
        Token::EQUALS => Object::TRUE,
        Token::NOT_EQUALS => Object::FALSE,
        _ => Object::NULL,
    }
}

fn evaluate_opposite_boolean(operator_token: Token) -> Object {
    match operator_token {
        Token::EQUALS => Object::FALSE,
        Token::NOT_EQUALS => Object::TRUE,
        _ => Object::NULL,
    }
}

#[cfg(test)]
mod tests;
