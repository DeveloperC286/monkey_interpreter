use crate::evaluator::object::Object;
use crate::syntax_analysis::abstract_syntax_tree::syntax_tree_node::*;

pub fn evaluate(expression: Expression) -> Object {
    Object::RETURN {
        value: Box::new(crate::evaluator::evaluate_expression(expression)),
    }
}

#[cfg(test)]
mod tests;