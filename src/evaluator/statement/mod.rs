use crate::evaluator::model::evaluation_error::EvaluationError;
use crate::evaluator::model::object::Object;
use crate::evaluator::Evaluator;
use crate::syntax_analysis::model::syntax_tree_node::*;

mod let_statement;
mod return_statement;

impl Evaluator {
    pub(super) fn evaluate_statement(
        &mut self,
        statement: Statement,
    ) -> Result<Object, EvaluationError> {
        match statement {
            Statement::Let {
                identifier,
                expression,
            } => self.evaluate_let_statement(identifier, *expression),
            Statement::Return { expression } => self.evaluate_return_statement(*expression),
        }
    }
}
