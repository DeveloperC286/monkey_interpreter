use model::object::Object;

use crate::evaluator::evaluator_context::EvaluatorContext;
use crate::syntax_analysis::model::abstract_syntax_tree::AbstractSyntaxTree;
use crate::syntax_analysis::model::abstract_syntax_tree::syntax_tree_node::*;

pub mod model;
pub mod evaluator_context;
mod expression;
mod statement;

pub fn evaluate(
    mut evaluator_context: EvaluatorContext,
    abstract_syntax_tree: AbstractSyntaxTree,
) -> (EvaluatorContext, Object) {
    let mut object = Object::NULL;

    if !abstract_syntax_tree.syntax_parsing_errors.is_empty() {
        panic!("Syntax errors unable to evaluate.");
    }

    for syntax_tree_node in abstract_syntax_tree.abstract_syntax_tree {
        let (returned_evaluator_context, returned_object) =
            evaluate_node(evaluator_context, syntax_tree_node);
        evaluator_context = returned_evaluator_context;
        object = returned_object;

        match object.clone() {
            Object::RETURN { value } => {
                object = *value;
                break;
            }
            Object::TYPE_MISMATCH | Object::UNKNOWN_OPERATOR => break,
            _ => {}
        }
    }

    (evaluator_context, object)
}

fn evaluate_block(
    mut evaluator_context: EvaluatorContext,
    block: Block,
) -> (EvaluatorContext, Object) {
    let mut object = Object::NULL;

    for syntax_tree_node in block.nodes {
        let (returned_evaluator_context, returned_object) =
            evaluate_node(evaluator_context, syntax_tree_node);
        evaluator_context = returned_evaluator_context;
        object = returned_object;

        match object.clone() {
            Object::RETURN { value: _ } | Object::TYPE_MISMATCH | Object::UNKNOWN_OPERATOR => break,
            _ => {}
        }
    }

    (evaluator_context, object)
}

fn evaluate_node(
    evaluator_context: EvaluatorContext,
    syntax_tree_node: SyntaxTreeNode,
) -> (EvaluatorContext, Object) {
    match syntax_tree_node {
        SyntaxTreeNode::EXPRESSION { expression } => {
            crate::evaluator::expression::evaluate(evaluator_context, expression)
        }
        SyntaxTreeNode::STATEMENT { statement } => {
            crate::evaluator::statement::evaluate(evaluator_context, statement)
        }
    }
}

#[cfg(test)]
mod tests;
