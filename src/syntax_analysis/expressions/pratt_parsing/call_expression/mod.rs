use std::iter::Peekable;
use std::slice::Iter;

use crate::lexical_analysis::token::Token;
use crate::syntax_analysis::abstract_syntax_tree::syntax_tree_node::{
    Expression, ExpressionPrecedence,
};

pub fn parse_call_expression(
    mut iterator: Peekable<Iter<Token>>,
    mut syntax_parsing_errors: Vec<String>,
    function: Expression,
) -> (Peekable<Iter<Token>>, Vec<String>, Option<Expression>) {
    debug!("Parsing a call expression.");

    // parse call expression
    let (returned_iterator, returned_syntax_parsing_errors, arguments) =
        parse_arguments(iterator, syntax_parsing_errors);
    iterator = returned_iterator;
    syntax_parsing_errors = returned_syntax_parsing_errors;

    // check call expression was correctly called
    match &function {
        Expression::IDENTIFIER {
            identifier_token: _,
        } => {}
        _ => {
            error!("parse_call_expression called with the function not being an Expression::IDENTIFIER.");
            return (iterator, syntax_parsing_errors, None);
        }
    }

    (
        iterator,
        syntax_parsing_errors,
        Some(Expression::CALL {
            function: Box::new(function),
            arguments,
        }),
    )
}

fn parse_arguments(
    mut iterator: Peekable<Iter<Token>>,
    mut syntax_parsing_errors: Vec<String>,
) -> (Peekable<Iter<Token>>, Vec<String>, Vec<Expression>) {
    debug!("Parsing arguments.");

    assert_token!(
        iterator,
        syntax_parsing_errors,
        Token::OPENING_ROUND_BRACKET,
        vec![]
    );
    let mut arguments = vec![];

    if let Some(token) = iterator.peek() {
        if **token != Token::CLOSING_ROUND_BRACKET {
            loop {
                match super::super::get_expression(
                    iterator,
                    syntax_parsing_errors,
                    ExpressionPrecedence::LOWEST,
                ) {
                    (returned_iterator, returned_syntax_parsing_errors, Some(expression)) => {
                        arguments.push(expression);
                        iterator = returned_iterator;
                        syntax_parsing_errors = returned_syntax_parsing_errors;
                    }
                    (returned_iterator, returned_syntax_parsing_errors, None) => {
                        syntax_parsing_errors = returned_syntax_parsing_errors;
                        syntax_parsing_errors
                            .push("Unable to parse expression in arguments.".to_string());
                        return (returned_iterator, syntax_parsing_errors, vec![]);
                    }
                }

                match iterator.peek() {
                    Some(token) => match token {
                        Token::CLOSING_ROUND_BRACKET => break,
                        Token::COMMA => {
                            iterator.next();
                        }
                        _ => {
                            return (iterator, syntax_parsing_errors, vec![]);
                        }
                    },
                    None => {
                        return (iterator, syntax_parsing_errors, vec![]);
                    }
                }
            }
        }
    }

    assert_token!(
        iterator,
        syntax_parsing_errors,
        Token::CLOSING_ROUND_BRACKET,
        vec![]
    );
    (iterator, syntax_parsing_errors, arguments)
}

#[cfg(test)]
mod tests;
