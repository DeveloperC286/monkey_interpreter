use std::iter::Peekable;
use std::slice::Iter;

use crate::lexical_analysis::token::{Token, TokenType};
use crate::syntax_analysis::abstract_syntax_tree::syntax_tree_node::{
    Expression, ExpressionPrecedence,
};

pub fn parse_grouped_expression(
    mut iterator: Peekable<Iter<Token>>,
    mut syntax_parsing_errors: Vec<String>,
) -> (Peekable<Iter<Token>>, Vec<String>, Option<Expression>) {
    debug!("Parsing a grouped expression.");

    assert_token!(
        iterator,
        syntax_parsing_errors,
        TokenType::OPENING_ROUND_BRACKET,
        None
    );
    let (returned_iterator, returned_syntax_parsing_errors, grouped_expression) =
        super::get_expression(
            iterator,
            syntax_parsing_errors,
            ExpressionPrecedence::LOWEST,
        );
    iterator = returned_iterator;
    syntax_parsing_errors = returned_syntax_parsing_errors;
    assert_token!(
        iterator,
        syntax_parsing_errors,
        TokenType::CLOSING_ROUND_BRACKET,
        None
    );

    return (iterator, syntax_parsing_errors, grouped_expression);
}
