use crate::evaluator::model::evaluation_error::EvaluationError;
use crate::evaluator::model::object::Object;
use crate::evaluator::Evaluator;
use crate::syntax_analysis::model::syntax_tree_node::Block;

impl Evaluator {
    pub(super) fn evaluate_function_expression(
        &self,
        parameters: Vec<String>,
        block: Block,
    ) -> Result<Object, EvaluationError> {
        Ok(Object::Function { parameters, block })
    }
}

#[cfg(test)]
mod tests;
