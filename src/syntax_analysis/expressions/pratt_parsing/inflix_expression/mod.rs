use std::iter::Peekable;
use std::slice::Iter;

use crate::lexical_analysis::token::Token;
use crate::syntax_analysis::abstract_syntax_tree::syntax_tree_node::Expression;
use crate::syntax_analysis::expression_precedence;

pub fn parse_inflix_expression(
    mut iterator: Peekable<Iter<Token>>,
    mut syntax_parsing_errors: Vec<String>,
    left_hand_expression: Expression,
) -> (Peekable<Iter<Token>>, Vec<String>, Option<Expression>) {
    debug!("Parsing a inflix expression.");

    let operator_token = match iterator.next() {
        Some(token) => token,
        None => return (iterator, syntax_parsing_errors, None),
    };
    let precedence =
        expression_precedence::get_current_expression_precedence(&operator_token.token_type);

    return match super::super::get_expression(iterator, syntax_parsing_errors, precedence) {
        (returned_iterator, returned_syntax_parsing_errors, Some(right_hand_expression)) => (
            returned_iterator,
            returned_syntax_parsing_errors,
            Some(Expression::INFIX {
                left_hand_expression: Box::new(left_hand_expression),
                operator_token: operator_token.clone(),
                right_hand_expression: Box::new(right_hand_expression),
            }),
        ),
        (returned_iterator, returned_syntax_parsing_errors, None) => {
            (returned_iterator, returned_syntax_parsing_errors, None)
        }
    };
}
