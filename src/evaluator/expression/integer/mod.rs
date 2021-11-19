use crate::evaluator::model::object::Object;
use crate::evaluator::Evaluator;
use crate::lexical_analysis::model::token::Token;

impl Evaluator {
    pub(super) fn evaluate_integer(&self, integer_token: Token) -> Object {
        match integer_token {
            Token::Integer { literal } => Object::Integer {
                value: literal.parse().unwrap(),
            },
            _ => panic!("Integer token not a integer token."),
        }
    }
}

#[cfg(test)]
mod tests;
