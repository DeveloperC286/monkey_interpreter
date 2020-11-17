use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EvaluatorContext {}

impl EvaluatorContext {
    pub fn new() -> EvaluatorContext {
        EvaluatorContext {}
    }
}
