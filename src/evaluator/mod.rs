use object::Object;

use crate::syntax_analysis::abstract_syntax_tree::syntax_tree_node::*;
use crate::syntax_analysis::abstract_syntax_tree::AbstractSyntaxTree;

pub mod object;
#[macro_use]
pub mod macros;
mod boolean;
mod infix;
mod integer;
mod prefix;

pub fn evaluate(abstract_syntax_tree: AbstractSyntaxTree) -> Object {
    let mut object = Object::NULL;

    for syntax_tree_node in abstract_syntax_tree.abstract_syntax_tree {
        object = evaluate_node(syntax_tree_node);
    }

    object
}

pub fn evaluate_node(syntax_tree_node: SyntaxTreeNode) -> Object {
    match syntax_tree_node {
        SyntaxTreeNode::EXPRESSION { expression } => match expression {
            Expression::INTEGER { integer_token } => integer::parse_integer(integer_token),
            Expression::BOOLEAN { boolean_token } => boolean::parse_boolean(boolean_token),
            Expression::PREFIX {
                prefix_token,
                right_hand_node,
            } => prefix::parse_prefix(prefix_token, *right_hand_node),
            Expression::INFIX {
                left_hand_node,
                operator_token,
                right_hand_node,
            } => infix::parse_infix(*left_hand_node, operator_token, *right_hand_node),
            _ => Object::NULL,
        },
        _ => Object::NULL,
    }
}

#[cfg(test)]
mod tests;
