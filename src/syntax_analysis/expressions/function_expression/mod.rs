use std::iter::Peekable;
use std::slice::Iter;

use crate::lexical_analysis::token::Token;
use crate::syntax_analysis::abstract_syntax_tree::syntax_tree_node::{
    Expression, ExpressionPrecedence,
};

pub fn parse_function_expression(
    mut iterator: Peekable<Iter<Token>>,
    mut syntax_parsing_errors: Vec<String>,
) -> (Peekable<Iter<Token>>, Vec<String>, Option<Expression>) {
    debug!("Parsing a function expression.");

    // parse function expression
    assert_token!(iterator, syntax_parsing_errors, Token::FUNCTION, None);
    let (returned_iterator, returned_syntax_parsing_errors, parameters) =
        parse_parameters(iterator, syntax_parsing_errors);
    iterator = returned_iterator;
    syntax_parsing_errors = returned_syntax_parsing_errors;

    // check function expression was parsed correctly
    let block = match crate::syntax_analysis::expressions::utilities::parse_block(iterator, syntax_parsing_errors) {
        (returned_iterator, returned_syntax_parsing_errors, Some(block)) => {
            iterator = returned_iterator;
            syntax_parsing_errors = returned_syntax_parsing_errors;
            block
        }
        (returned_iterator, returned_syntax_parsing_errors, None) => {
            error!("parse_function_expression could not parse the functions block.");
            return (returned_iterator, returned_syntax_parsing_errors, None);
        }
    };

    (
        iterator,
        syntax_parsing_errors,
        Some(Expression::FUNCTION {
            parameters,
            block: Box::new(block),
        }),
    )
}

fn parse_parameters(
    mut iterator: Peekable<Iter<Token>>,
    mut syntax_parsing_errors: Vec<String>,
) -> (Peekable<Iter<Token>>, Vec<String>, Vec<Expression>) {
    debug!("Parsing parameters.");

    assert_token!(
        iterator,
        syntax_parsing_errors,
        Token::OPENING_ROUND_BRACKET,
        vec![]
    );
    let mut parameters = vec![];

    if let Some(token) = iterator.peek() {
        if **token != Token::CLOSING_ROUND_BRACKET {
            loop {
                match super::get_expression(
                    iterator,
                    syntax_parsing_errors,
                    ExpressionPrecedence::LOWEST,
                ) {
                    (returned_iterator, returned_syntax_parsing_errors, Some(expression)) => {
                        match expression.clone() {
                            Expression::IDENTIFIER {
                                identifier_token: _,
                            } => {
                                parameters.push(expression);
                                iterator = returned_iterator;
                                syntax_parsing_errors = returned_syntax_parsing_errors;
                            }
                            _ => {
                                iterator = returned_iterator;
                                syntax_parsing_errors = returned_syntax_parsing_errors;
                                syntax_parsing_errors.push(
                                    "Only allowed Expression::IDENTIFIER in parameters."
                                        .to_string(),
                                );
                            }
                        }
                    }
                    (returned_iterator, returned_syntax_parsing_errors, None) => {
                        syntax_parsing_errors = returned_syntax_parsing_errors;
                        syntax_parsing_errors
                            .push("Unable to parse expression in parameters.".to_string());
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
                            syntax_parsing_errors.push(
                                "Parameters must be comma seperated identifiers.".to_string(),
                            );
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
    (iterator, syntax_parsing_errors, parameters)
}

#[cfg(test)]
mod tests;
