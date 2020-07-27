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
                        _ => Object::NULL,
                    }
                }
                _ => Object::NULL,
            }
        }
        _ => Object::NULL,
    };
}

#[cfg(test)]
mod tests;
