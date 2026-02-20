use crate::evaluator::model::object::Object;
use crate::evaluator::Evaluator;
use crate::syntax_analysis::model::syntax_tree_node::{Expression, InfixOperator};

impl Evaluator {
    pub(super) fn evaluate_infix_expression(
        &mut self,
        left_hand: Expression,
        operator: InfixOperator,
        right_hand: Expression,
    ) -> anyhow::Result<Object> {
        fn evaluate_same_boolean(operator: InfixOperator) -> anyhow::Result<Object> {
            match operator {
                InfixOperator::Equals => Ok(Object::True),
                InfixOperator::NotEquals => Ok(Object::False),
                _ => anyhow::bail!("UnknownOperator"),
            }
        }

        fn evaluate_opposite_boolean(operator: InfixOperator) -> anyhow::Result<Object> {
            match operator {
                InfixOperator::Equals => Ok(Object::False),
                InfixOperator::NotEquals => Ok(Object::True),
                _ => anyhow::bail!("UnknownOperator"),
            }
        }

        match self.evaluate_expression(left_hand)? {
            Object::Integer { value: left_value } => match self.evaluate_expression(right_hand)? {
                Object::Integer { value: right_value } => match operator {
                    InfixOperator::Plus => Ok(Object::Integer {
                        value: left_value + right_value,
                    }),
                    InfixOperator::Minus => Ok(Object::Integer {
                        value: left_value - right_value,
                    }),
                    InfixOperator::Multiply => Ok(Object::Integer {
                        value: left_value * right_value,
                    }),
                    InfixOperator::Divide => Ok(Object::Integer {
                        value: left_value / right_value,
                    }),
                    InfixOperator::GreaterThan => match left_value > right_value {
                        true => Ok(Object::True),
                        false => Ok(Object::False),
                    },
                    InfixOperator::LesserThan => match left_value < right_value {
                        true => Ok(Object::True),
                        false => Ok(Object::False),
                    },
                    InfixOperator::Equals => match left_value == right_value {
                        true => Ok(Object::True),
                        false => Ok(Object::False),
                    },
                    InfixOperator::NotEquals => match left_value != right_value {
                        true => Ok(Object::True),
                        false => Ok(Object::False),
                    },
                },
                _ => anyhow::bail!("TypeMismatch"),
            },
            Object::True => match self.evaluate_expression(right_hand)? {
                Object::True => evaluate_same_boolean(operator),
                Object::False => evaluate_opposite_boolean(operator),
                _ => anyhow::bail!("TypeMismatch"),
            },
            Object::False => match self.evaluate_expression(right_hand)? {
                Object::False => evaluate_same_boolean(operator),
                Object::True => evaluate_opposite_boolean(operator),
                _ => anyhow::bail!("TypeMismatch"),
            },
            Object::String { value: left_value } => match self.evaluate_expression(right_hand)? {
                Object::String { value: right_value } => match operator {
                    InfixOperator::Equals => match left_value.eq(&right_value) {
                        true => Ok(Object::True),
                        false => Ok(Object::False),
                    },
                    InfixOperator::NotEquals => match left_value.ne(&right_value) {
                        true => Ok(Object::True),
                        false => Ok(Object::False),
                    },
                    InfixOperator::Plus => {
                        let mut concatenated = left_value;
                        concatenated.push_str(&right_value);
                        Ok(Object::String {
                            value: concatenated,
                        })
                    }
                    _ => anyhow::bail!("UnknownOperator"),
                },
                _ => anyhow::bail!("TypeMismatch"),
            },
            _ => anyhow::bail!("UnknownOperator"),
        }
    }
}
