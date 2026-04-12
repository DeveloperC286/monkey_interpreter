use crate::evaluator::Object;
use crate::evaluator::Evaluator;
use crate::syntax_analysis::Block;

impl Evaluator {
    pub(super) fn evaluate_function_expression(
        &self,
        parameters: Vec<String>,
        block: Block,
    ) -> anyhow::Result<Object> {
        Ok(Object::Function { parameters, block })
    }
}
