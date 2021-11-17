use model::evaluator_context::EvaluatorContext;
use model::object::Object;

use crate::syntax_analysis::model::abstract_syntax_tree::syntax_tree_node::*;
use crate::syntax_analysis::model::abstract_syntax_tree::AbstractSyntaxTree;

pub(crate) mod model;

#[cfg(test)]
#[macro_use]
mod tests;

mod expression;
mod statement;

pub(crate) fn evaluate(
    mut evaluator_context: EvaluatorContext,
    abstract_syntax_tree: AbstractSyntaxTree,
) -> (EvaluatorContext, Object) {
    let mut object = Object::Null;

    if !abstract_syntax_tree.syntax_parsing_errors.is_empty() {
        panic!("Syntax errors unable to evaluate.");
    }

    for syntax_tree_node in abstract_syntax_tree.abstract_syntax_tree {
        let (returned_evaluator_context, returned_object) =
            evaluate_node(evaluator_context, syntax_tree_node);
        evaluator_context = returned_evaluator_context;
        object = returned_object;

        match object.clone() {
            Object::Return { value } => {
                object = *value;
                break;
            }
            Object::TypeMismatch | Object::UnknownOperator => break,
            _ => {}
        }
    }

    (evaluator_context, object)
}

fn evaluate_block(
    mut evaluator_context: EvaluatorContext,
    block: Block,
) -> (EvaluatorContext, Object) {
    let mut object = Object::Null;

    for syntax_tree_node in block.nodes {
        let (returned_evaluator_context, returned_object) =
            evaluate_node(evaluator_context, syntax_tree_node);
        evaluator_context = returned_evaluator_context;
        object = returned_object;

        match object.clone() {
            Object::Return { value: _ } | Object::TypeMismatch | Object::UnknownOperator => break,
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
        SyntaxTreeNode::Expression { expression } => {
            crate::evaluator::expression::evaluate(evaluator_context, expression)
        }
        SyntaxTreeNode::Statement { statement } => {
            crate::evaluator::statement::evaluate(evaluator_context, statement)
        }
    }
}
