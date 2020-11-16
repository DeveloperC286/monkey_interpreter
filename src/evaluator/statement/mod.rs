use crate::evaluator::object::Object;
use crate::syntax_analysis::abstract_syntax_tree::syntax_tree_node::*;

mod return_statement;

pub fn evaluate(statement: Statement) -> Object {
    match statement {
        Statement::RETURN { expression } => crate::evaluator::statement::return_statement::evaluate(*expression),
        _ => Object::NULL,
    }
}
