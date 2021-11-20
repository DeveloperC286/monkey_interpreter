use crate::evaluator::model::object::{ErrorType, Object};
use crate::evaluator::Evaluator;
use crate::lexical_analysis::model::token::Token;
use crate::syntax_analysis::model::abstract_syntax_tree::syntax_tree_node::Expression;

impl Evaluator {
    pub(super) fn evaluate_prefix_expression(
        &mut self,
        prefix_token: Token,
        right_hand_expression: Expression,
    ) -> Object {
        let object = self.evaluate_expression(right_hand_expression);
        match prefix_token {
            Token::Not => match object {
                Object::True => Object::False,
                Object::False => Object::True,
                _ => Object::Error {
                    error_type: ErrorType::UnknownOperator,
                },
            },
            Token::Minus => match object {
                Object::Integer { value } => Object::Integer { value: -value },
                _ => Object::Error {
                    error_type: ErrorType::UnknownOperator,
                },
            },
            _ => panic!("Prefix token not a prefix token."),
        }
    }
}

#[cfg(test)]
mod tests;
