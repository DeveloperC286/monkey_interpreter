use crate::evaluator::model::evaluation_error::EvaluationError;
use crate::evaluator::model::object::Object;
use crate::evaluator::Evaluator;
use crate::lexical_analysis::model::token::Token;

impl Evaluator {
    pub(super) fn evaluate_integer(&self, integer_token: Token) -> Result<Object, EvaluationError> {
        match integer_token {
            Token::Integer { literal } => Ok(Object::Integer { value: literal }),
            _ => Err(EvaluationError::NotIntegerToken),
        }
    }
}

#[cfg(test)]
mod tests;
