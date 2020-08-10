use crate::evaluator::object::Object;
use crate::syntax_analysis::abstract_syntax_tree::syntax_tree_node::*;

pub fn evaluate(condition: Expression, consequence: Block, alternative: Option<Block>) -> Object {
    match crate::evaluator::evaluate_expression(condition) {
        Object::NULL | Object::FALSE => match alternative {
            Some(block) => match block {
                Block::BLOCK { blocks } => crate::evaluator::evaluate_nodes(blocks),
            },
            None => Object::NULL,
        },
        _ => match consequence {
            Block::BLOCK { blocks } => crate::evaluator::evaluate_nodes(blocks),
        },
    }
}

#[cfg(test)]
mod tests;
