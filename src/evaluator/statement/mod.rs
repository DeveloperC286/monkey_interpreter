use crate::evaluator::model::object::Object;
use crate::evaluator::Evaluator;
use crate::syntax_analysis::model::abstract_syntax_tree::syntax_tree_node::*;

mod let_statement;
mod return_statement;

impl Evaluator {
    pub(super) fn evaluate_statement(&mut self, statement: Statement) -> Object {
        match statement {
            Statement::Let {
                identifier,
                expression,
            } => self.evaluate_let_statement(identifier, *expression),
            Statement::Return { expression } => self.evaluate_return_statement(*expression),
        }
    }
}
