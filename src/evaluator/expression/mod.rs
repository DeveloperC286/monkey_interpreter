use crate::evaluator::model::evaluation_error::EvaluationError;
use crate::evaluator::model::object::Object;
use crate::evaluator::Evaluator;
use crate::syntax_analysis::model::syntax_tree_node::*;

mod boolean;
mod call;
mod function;
mod if_expression;
mod infix;
mod integer;
mod prefix;
mod string;

impl Evaluator {
    pub(crate) fn evaluate_expression(
        &mut self,
        expression: Expression,
    ) -> Result<Object, EvaluationError> {
        match expression {
            Expression::Integer { literal } => self.evaluate_integer(literal),
            Expression::String { literal } => self.evaluate_string(literal),
            Expression::Boolean { literal } => self.evaluate_boolean(literal),
            Expression::NotPrefix { right_hand } => {
                self.evaluate_not_prefix_expression(*right_hand)
            }
            Expression::MinusPrefix { right_hand } => {
                self.evaluate_minus_prefix_expression(*right_hand)
            }
            Expression::PlusInfix {
                left_hand,
                right_hand,
            } => self.evaluate_plus_infix_expression(*left_hand, *right_hand),
            Expression::MinusInfix {
                left_hand,
                right_hand,
            } => self.evaluate_minus_infix_expression(*left_hand, *right_hand),
            Expression::Infix {
                left_hand,
                operator,
                right_hand,
            } => self.evaluate_infix_expression(*left_hand, operator, *right_hand),
            Expression::If {
                condition,
                consequence,
                alternative,
            } => self.evaluate_if_expression(*condition, *consequence, *alternative),
            Expression::Identifier { identifier } => Ok(self.environment.get(identifier)),
            Expression::Function { parameters, block } => {
                self.evaluate_function_expression(parameters, *block)
            }
            Expression::Call {
                function,
                arguments,
            } => self.evaluate_call_expression(*function, arguments),
        }
    }
}
