use crate::evaluator::model::evaluation_error::EvaluationError;
use crate::evaluator::model::object::Object;
use crate::evaluator::Evaluator;
use crate::syntax_analysis::model::syntax_tree_node::*;

impl Evaluator {
    pub(super) fn evaluate_let_statement(
        &mut self,
        identifier: String,
        expression: Expression,
    ) -> Result<Object, EvaluationError> {
        let expression = self.evaluate_expression(expression)?;

        if let Object::Return { value: _ } = expression {
            return Err(EvaluationError::UnassignableObject);
        }

        self.environment.set(identifier, expression);

        Ok(Object::Null)
    }
}
