use object::Object;

use crate::lexical_analysis::token::Token;
use crate::syntax_analysis::abstract_syntax_tree::AbstractSyntaxTree;
use crate::syntax_analysis::abstract_syntax_tree::syntax_tree_node::*;

pub mod object;
#[macro_use]
pub mod macros;

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
            Expression::INTEGER { integer_token } => {
                match integer_token {
                    Token::INTEGER { literal } => {
                        Object::INTEGER { value: literal.parse().unwrap() }
                    }
                    _ => Object::NULL
                }
            }
            _ => Object::NULL
        },
        _ => Object::NULL
    };
}

#[cfg(test)]
mod tests;