use crate::evaluator::object::Object;
use crate::syntax_analysis::abstract_syntax_tree::syntax_tree_node::*;

mod boolean;
mod if_expression;
mod infix;
mod integer;
mod prefix;

pub fn evaluate(expression: Expression) -> Object {
    match expression {
        Expression::INTEGER { integer_token } => integer::evaluate(integer_token),
        Expression::BOOLEAN { boolean_token } => boolean::evaluate(boolean_token),
        Expression::PREFIX {
            prefix_token,
            right_hand,
        } => prefix::evaluate(prefix_token, *right_hand),
        Expression::INFIX {
            left_hand,
            operator_token,
            right_hand,
        } => infix::evaluate(*left_hand, operator_token, *right_hand),
        Expression::IF {
            condition,
            consequence,
            alternative,
        } => if_expression::evaluate(*condition, *consequence, *alternative),
        _ => Object::NULL,
    }
}
