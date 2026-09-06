use std::rc::Rc;

use crate::evaluator::Evaluator;
use crate::evaluator::Object;
use crate::syntax_analysis::Block;

impl Evaluator {
    pub(super) fn evaluate_function_expression(
        &self,
        parameters: &[String],
        block: &Rc<Block>,
    ) -> anyhow::Result<Object> {
        Ok(Object::Function {
            parameters: parameters.to_vec(),
            block: Rc::clone(block),
            // Captured when the function expression is evaluated, so the function's free
            // variables resolve within the scope it was defined in however far it travels.
            scope: self.environment.current(),
        })
    }
}
