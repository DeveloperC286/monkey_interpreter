use crate::evaluator::object::Object;
use crate::lexical_analysis::token::Token;
use crate::syntax_analysis::abstract_syntax_tree::syntax_tree_node::*;

pub fn evaluate(prefix_token: Token, right_hand_expression: Expression) -> Object {
    let object = crate::evaluator::evaluate_expression(right_hand_expression);
    match prefix_token {
        Token::NOT => match object {
            Object::TRUE => Object::FALSE,
            Object::FALSE => Object::TRUE,
            _ => Object::NULL,
        },
        Token::MINUS => match object {
            Object::INTEGER { value } => Object::INTEGER { value: -value },
            _ => Object::NULL,
        },
        _ => Object::NULL,
    }
}

#[cfg(test)]
mod tests;
