use crate::evaluator::model::evaluation_error::EvaluationError;
use crate::evaluator::model::object::Object;
use crate::evaluator::Evaluator;
use crate::lexical_analysis::model::token::Token;
use crate::syntax_analysis::model::syntax_tree_node::Expression;

impl Evaluator {
    pub(super) fn evaluate_prefix_expression(
        &mut self,
        prefix_token: Token,
        right_hand_expression: Expression,
    ) -> Result<Object, EvaluationError> {
        let object = self.evaluate_expression(right_hand_expression)?;

        match prefix_token {
            Token::Not => match object {
                Object::True => Ok(Object::False),
                Object::False => Ok(Object::True),
                _ => Err(EvaluationError::UnknownOperator),
            },
            Token::Minus => match object {
                Object::Integer { value } => Ok(Object::Integer { value: -value }),
                _ => Err(EvaluationError::UnknownOperator),
            },
            _ => Err(EvaluationError::NotPrefixToken),
        }
    }
}

#[cfg(test)]
mod tests;
