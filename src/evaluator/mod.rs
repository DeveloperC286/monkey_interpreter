use object::Object;

use crate::lexical_analysis::token::Token;
use crate::syntax_analysis::abstract_syntax_tree::syntax_tree_node::*;
use crate::syntax_analysis::abstract_syntax_tree::AbstractSyntaxTree;

pub mod object;
#[macro_use]
pub mod macros;
mod boolean;
mod integer;

pub fn evaluate(abstract_syntax_tree: AbstractSyntaxTree) -> Object {
    let mut object = Object::NULL;

    for syntax_tree_node in abstract_syntax_tree.abstract_syntax_tree {
        object = evaluate_node(syntax_tree_node);
    }

    return object;
}

fn evaluate_node(syntax_tree_node: SyntaxTreeNode) -> Object {
    return match syntax_tree_node {
        SyntaxTreeNode::EXPRESSION { expression } => match expression {
            Expression::INTEGER { integer_token } => integer::parse_integer_token(integer_token),
            Expression::BOOLEAN { boolean_token } => boolean::parse_boolean_token(boolean_token),
            _ => Object::NULL,
        },
        _ => Object::NULL,
    };
}

#[cfg(test)]
mod tests;
