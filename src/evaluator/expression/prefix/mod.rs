use crate::evaluator::object::Object;
use crate::lexical_analysis::token::Token;
use crate::syntax_analysis::abstract_syntax_tree::syntax_tree_node::Expression;

pub fn evaluate(prefix_token: Token, right_hand_expression: Expression) -> Object {
    let object = crate::evaluator::expression::evaluate(right_hand_expression);
    match prefix_token {
        Token::NOT => match object {
            Object::TRUE => Object::FALSE,
            Object::FALSE => Object::TRUE,
            _ => Object::UNKNOWN_OPERATOR,
        },
        Token::MINUS => match object {
            Object::INTEGER { value } => Object::INTEGER { value: -value },
            _ => Object::UNKNOWN_OPERATOR,
        },
        _ => panic!("Prefix token not a prefix token."),
    }
}

#[cfg(test)]
mod tests;