use crate::evaluator::object::Object;
use crate::lexical_analysis::token::Token;
use crate::syntax_analysis::abstract_syntax_tree::AbstractSyntaxTree;
use crate::syntax_analysis::abstract_syntax_tree::syntax_tree_node::*;

pub fn parse_boolean_token(boolean_token: Token) -> Object {
    match boolean_token {
        Token::TRUE => Object::BOOLEAN { value: true },
        Token::FALSE => Object::BOOLEAN { value: false },
        _ => Object::NULL
    }
}

#[cfg(test)]
mod tests;