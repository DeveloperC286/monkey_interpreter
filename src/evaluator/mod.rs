use crate::evaluator::object::Object;
use crate::syntax_analysis::abstract_syntax_tree::AbstractSyntaxTree;
use crate::syntax_analysis::abstract_syntax_tree::syntax_tree_node::*;

mod object;
mod return_statement;

mod expression;

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
        SyntaxTreeNode::STATEMENT { statement } => evaluate_statement(statement),
    }
}

fn evaluate_statement(statement: Statement) -> Object {
    match statement {
        Statement::RETURN { expression } => return_statement::evaluate(*expression),
        _ => Object::NULL,
    }
}

#[cfg(test)]
mod tests;
