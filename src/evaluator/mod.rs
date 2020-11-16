use crate::evaluator::object::Object;
use crate::syntax_analysis::abstract_syntax_tree::AbstractSyntaxTree;
use crate::syntax_analysis::abstract_syntax_tree::syntax_tree_node::*;

mod object;
mod expression;
mod statement;

pub fn evaluate(abstract_syntax_tree: AbstractSyntaxTree) -> Object {
    let mut object = Object::NULL;

    if !abstract_syntax_tree.syntax_parsing_errors.is_empty() {
        panic!("Syntax errors unable to evaluate.");
    }

    for syntax_tree_node in abstract_syntax_tree.abstract_syntax_tree {
        object = evaluate_node(syntax_tree_node);

        match object.clone() {
            Object::RETURN { value } => {
                object = *value;
                break;
            }
            Object::TYPE_MISMATCH | Object::UNKNOWN_OPERATOR => break,
            _ => {}
        }
    }

    object
}

fn evaluate_block(block: Block) -> Object {
    let mut object = Object::NULL;

    for syntax_tree_node in block.nodes {
        object = evaluate_node(syntax_tree_node);

        match object.clone() {
            Object::RETURN { value: _ } | Object::TYPE_MISMATCH | Object::UNKNOWN_OPERATOR => break,
            _ => {}
        }
    }

    object
}

fn evaluate_node(syntax_tree_node: SyntaxTreeNode) -> Object {
    match syntax_tree_node {
        SyntaxTreeNode::EXPRESSION { expression } => crate::evaluator::expression::evaluate(expression),
        SyntaxTreeNode::STATEMENT { statement } => crate::evaluator::statement::evaluate(statement),
    }
}

#[cfg(test)]
mod tests;
