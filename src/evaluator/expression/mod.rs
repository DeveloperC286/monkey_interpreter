use crate::evaluator::model::object::Object;
use crate::evaluator::Evaluator;
use crate::syntax_analysis::model::abstract_syntax_tree::syntax_tree_node::*;

mod boolean;
mod if_expression;
mod infix;
mod integer;
mod prefix;

impl Evaluator {
    pub(crate) fn evaluate_expression(&self, expression: Expression) -> Object {
        match expression {
            Expression::Integer { integer_token } => self.evaluate_integer(integer_token),
            Expression::Boolean { boolean_token } => self.evaluate_boolean(boolean_token),
            Expression::Prefix {
                prefix_token,
                right_hand,
            } => self.evaluate_prefix_expression(prefix_token, *right_hand),
            Expression::Infix {
                left_hand,
                operator_token,
                right_hand,
            } => self.evaluate_infix_expression(*left_hand, operator_token, *right_hand),
            Expression::If {
                condition,
                consequence,
                alternative,
            } => self.evaluate_if_expression(*condition, *consequence, *alternative),
            _ => Object::Null,
        }
    }
}
