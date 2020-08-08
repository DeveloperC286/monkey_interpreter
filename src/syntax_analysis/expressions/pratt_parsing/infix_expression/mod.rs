use std::iter::Peekable;
use std::slice::Iter;

use crate::lexical_analysis::token::Token;
use crate::syntax_analysis::abstract_syntax_tree::syntax_tree_node::{Expression, SyntaxTreeNode};
use crate::syntax_analysis::expression_precedence;

pub fn parse_infix_expression(
    mut iterator: Peekable<Iter<Token>>,
    syntax_parsing_errors: Vec<String>,
    left_hand: Expression,
) -> (Peekable<Iter<Token>>, Vec<String>, Option<Expression>) {
    debug!("Parsing a infix expression.");

    let operator_token = match iterator.next() {
        Some(token) => token,
        None => return (iterator, syntax_parsing_errors, None),
    };
    let precedence = expression_precedence::get_current_expression_precedence(&operator_token);

    match crate::syntax_analysis::expressions::get_expression_node(
        iterator,
        syntax_parsing_errors,
        precedence,
    ) {
        (returned_iterator, returned_syntax_parsing_errors, Some(right_hand_node)) => (
            returned_iterator,
            returned_syntax_parsing_errors,
            Some(Expression::INFIX {
                left_hand_node: Box::new(SyntaxTreeNode::EXPRESSION {
                    expression: left_hand,
                }),
                operator_token: operator_token.clone(),
                right_hand_node: Box::new(right_hand_node),
            }),
        ),
        (returned_iterator, returned_syntax_parsing_errors, None) => {
            (returned_iterator, returned_syntax_parsing_errors, None)
        }
    }
}

#[cfg(test)]
mod tests;
