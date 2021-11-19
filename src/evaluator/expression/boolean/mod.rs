use crate::evaluator::model::object::Object;
use crate::evaluator::Evaluator;
use crate::lexical_analysis::model::token::Token;

impl Evaluator {
    pub(super) fn evaluate_boolean(&self, boolean_token: Token) -> Object {
        match boolean_token {
            Token::True => Object::True,
            Token::False => Object::False,
            _ => panic!("Boolean token not a boolean token."),
        }
    }
}

#[cfg(test)]
mod tests;
