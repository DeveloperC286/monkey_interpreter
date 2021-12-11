use crate::evaluator::model::evaluation_error::EvaluationError;
use crate::evaluator::model::object::Object;
use crate::evaluator::Evaluator;
use crate::lexical_analysis::model::token::Token;

impl Evaluator {
    pub(super) fn evaluate_boolean(&self, boolean_token: Token) -> Result<Object, EvaluationError> {
        match boolean_token {
            Token::True => Ok(Object::True),
            Token::False => Ok(Object::False),
            _ => Err(EvaluationError::NotBooleanToken),
        }
    }
}

#[cfg(test)]
mod tests;
