use crate::evaluator::object::Object;
use crate::lexical_analysis::token::Token;
use crate::syntax_analysis::abstract_syntax_tree::syntax_tree_node::*;
use crate::syntax_analysis::abstract_syntax_tree::AbstractSyntaxTree;

pub fn parse_integer_token(integer_token: Token) -> Object {
    match integer_token {
        Token::INTEGER { literal } => Object::INTEGER {
            value: literal.parse().unwrap(),
        },
        _ => Object::NULL,
    }
}

#[cfg(test)]
mod tests;
