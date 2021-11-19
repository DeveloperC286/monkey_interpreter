use crate::evaluator::model::object::Object;
use crate::evaluator::Evaluator;
use crate::syntax_analysis::model::abstract_syntax_tree::syntax_tree_node::*;

mod return_statement;

impl Evaluator {
    pub(super) fn evaluate_statement(&self, statement: Statement) -> Object {
        match statement {
            Statement::Return { expression } => self.evaluate_return_statement(*expression),
            _ => Object::Null,
        }
    }
}
