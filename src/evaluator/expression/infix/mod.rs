use crate::evaluator::model::evaluation_error::EvaluationError;
use crate::evaluator::model::object::Object;
use crate::evaluator::Evaluator;
use crate::lexical_analysis::model::token::Token;
use crate::syntax_analysis::model::syntax_tree_node::Expression;

impl Evaluator {
    pub(super) fn evaluate_plus_infix_expression(
        &mut self,
        left_hand: Expression,
        right_hand: Expression,
    ) -> Result<Object, EvaluationError> {
        match self.evaluate_expression(left_hand)? {
            Object::Integer { value: left_value } => match self.evaluate_expression(right_hand)? {
                Object::Integer { value: right_value } => Ok(Object::Integer {
                    value: left_value + right_value,
                }),
                _ => Err(EvaluationError::TypeMismatch),
            },
            Object::String { value: left_value } => match self.evaluate_expression(right_hand)? {
                Object::String { value: right_value } => {
                    let mut concatenated = left_value;
                    concatenated.push_str(&right_value);
                    Ok(Object::String {
                        value: concatenated,
                    })
                }
                _ => Err(EvaluationError::TypeMismatch),
            },
            _ => Err(EvaluationError::TypeMismatch),
        }
    }

    pub(super) fn evaluate_infix_expression(
        &mut self,
        left_hand: Expression,
        operator_token: Token,
        right_hand: Expression,
    ) -> Result<Object, EvaluationError> {
        fn evaluate_same_boolean(operator_token: Token) -> Result<Object, EvaluationError> {
            match operator_token {
                Token::Equals => Ok(Object::True),
                Token::NotEquals => Ok(Object::False),
                _ => Err(EvaluationError::UnknownOperator),
            }
        }

        fn evaluate_opposite_boolean(operator_token: Token) -> Result<Object, EvaluationError> {
            match operator_token {
                Token::Equals => Ok(Object::False),
                Token::NotEquals => Ok(Object::True),
                _ => Err(EvaluationError::UnknownOperator),
            }
        }

        match self.evaluate_expression(left_hand)? {
            Object::Integer { value: left_value } => match self.evaluate_expression(right_hand)? {
                Object::Integer { value: right_value } => match operator_token {
                    Token::Minus => Ok(Object::Integer {
                        value: left_value - right_value,
                    }),
                    Token::Multiply => Ok(Object::Integer {
                        value: left_value * right_value,
                    }),
                    Token::Divide => Ok(Object::Integer {
                        value: left_value / right_value,
                    }),
                    Token::GreaterThan => match left_value > right_value {
                        true => Ok(Object::True),
                        false => Ok(Object::False),
                    },
                    Token::LesserThan => match left_value < right_value {
                        true => Ok(Object::True),
                        false => Ok(Object::False),
                    },
                    Token::Equals => match left_value == right_value {
                        true => Ok(Object::True),
                        false => Ok(Object::False),
                    },
                    Token::NotEquals => match left_value != right_value {
                        true => Ok(Object::True),
                        false => Ok(Object::False),
                    },
                    _ => Err(EvaluationError::UnknownOperator),
                },
                _ => Err(EvaluationError::TypeMismatch),
            },
            Object::True => match self.evaluate_expression(right_hand)? {
                Object::True => evaluate_same_boolean(operator_token),
                Object::False => evaluate_opposite_boolean(operator_token),
                _ => Err(EvaluationError::TypeMismatch),
            },
            Object::False => match self.evaluate_expression(right_hand)? {
                Object::False => evaluate_same_boolean(operator_token),
                Object::True => evaluate_opposite_boolean(operator_token),
                _ => Err(EvaluationError::TypeMismatch),
            },
            Object::String { value: left_value } => match self.evaluate_expression(right_hand)? {
                Object::String { value: right_value } => match operator_token {
                    Token::Equals => match left_value.eq(&right_value) {
                        true => Ok(Object::True),
                        false => Ok(Object::False),
                    },
                    Token::NotEquals => match left_value.ne(&right_value) {
                        true => Ok(Object::True),
                        false => Ok(Object::False),
                    },
                    _ => Err(EvaluationError::UnknownOperator),
                },
                _ => Err(EvaluationError::TypeMismatch),
            },
            _ => Err(EvaluationError::UnknownOperator),
        }
    }
}
