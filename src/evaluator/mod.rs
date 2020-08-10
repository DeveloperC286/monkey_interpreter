use object::Object;

use crate::syntax_analysis::abstract_syntax_tree::syntax_tree_node::*;
use crate::syntax_analysis::abstract_syntax_tree::AbstractSyntaxTree;

pub mod object;
#[macro_use]
pub mod macros;
mod boolean;
mod if_statement;
mod infix;
mod integer;
mod prefix;

pub fn evaluate(abstract_syntax_tree: AbstractSyntaxTree) -> Object {
    evaluate_nodes(abstract_syntax_tree.abstract_syntax_tree)
}

fn evaluate_nodes(syntax_tree_nodes: Vec<SyntaxTreeNode>) -> Object {
    let mut object = Object::NULL;

    for syntax_tree_node in syntax_tree_nodes {
        object = evaluate_node(syntax_tree_node);
    }

    object
}

fn evaluate_node(syntax_tree_node: SyntaxTreeNode) -> Object {
    match syntax_tree_node {
        SyntaxTreeNode::EXPRESSION { expression } => evaluate_expression(expression),
        SyntaxTreeNode::STATEMENT { statement } => evaluate_statement(statement),
    }
}

fn evaluate_statement(_statement: Statement) -> Object {
    Object::NULL
}

fn evaluate_expression(expression: Expression) -> Object {
    match expression {
        Expression::INTEGER { integer_token } => integer::evaluate(integer_token),
        Expression::BOOLEAN { boolean_token } => boolean::evaluate(boolean_token),
        Expression::PREFIX {
            prefix_token,
            right_hand,
        } => prefix::evaluate(prefix_token, *right_hand),
        Expression::INFIX {
            left_hand,
            operator_token,
            right_hand,
        } => infix::evaluate(*left_hand, operator_token, *right_hand),
        Expression::IF {
            condition,
            consequence,
            alternative,
        } => if_statement::evaluate(*condition, *consequence, *alternative),
        _ => Object::NULL,
    }
}

#[cfg(test)]
mod tests;
