use std::iter::Peekable;
use std::slice::Iter;

use crate::lexical_analysis::token::Token;
use crate::syntax_analysis::abstract_syntax_tree::syntax_tree_node::{
    Expression, ExpressionPrecedence, SyntaxTreeNode,
};

pub mod function_expression;
pub mod grouped_expression;
pub mod if_expression;
pub mod pratt_parsing;
pub mod utilities;

pub fn parse_expression(
    mut iterator: Peekable<Iter<Token>>,
    mut syntax_parsing_errors: Vec<String>,
    expression_precedence: ExpressionPrecedence,
) -> (Peekable<Iter<Token>>, Vec<String>, Option<SyntaxTreeNode>) {
    let (returned_iterator, returned_syntax_parsing_errors, expression_option) = get_expression(
        iterator,
        syntax_parsing_errors,
        expression_precedence,
    );
    iterator = returned_iterator;
    syntax_parsing_errors = returned_syntax_parsing_errors;

    semicolon!(iterator);

    match expression_option {
        Some(expression) => (
            iterator,
            syntax_parsing_errors,
            Some(SyntaxTreeNode::EXPRESSION { expression }),
        ),
        None => (iterator, syntax_parsing_errors, None),
    }
}

pub fn get_expression(
    mut iterator: Peekable<Iter<Token>>,
    mut syntax_parsing_errors: Vec<String>,
    expression_precedence: ExpressionPrecedence,
) -> (Peekable<Iter<Token>>, Vec<String>, Option<Expression>) {
    debug!("Parsing an expression.");

    let mut expression: Option<Expression> = None;

    match iterator.peek() {
        Some(token) => match token {
            Token::IDENTIFIER { literal } => {
                debug!("Found an identifier expression.");
                expression = Some(Expression::IDENTIFIER {
                    identifier_token: iterator.next().unwrap().clone(),
                });
            }
            Token::INTEGER { literal } => {
                debug!("Found an integer expression.");
                expression = Some(Expression::INTEGER {
                    integer_token: iterator.next().unwrap().clone(),
                });
            }
            Token::NOT | Token::MINUS => {
                debug!("Found a prefix expression.");
                let token = iterator.next().unwrap().clone();

                match parse_expression(
                    iterator,
                    syntax_parsing_errors,
                    ExpressionPrecedence::PREFIX,
                ) {
                    (
                        returned_iterator,
                        returned_syntax_parsing_errors,
                        Some(right_hand_expression),
                    ) => {
                        iterator = returned_iterator;
                        syntax_parsing_errors = returned_syntax_parsing_errors;
                        expression = Some(Expression::PREFIX {
                            prefix_token: token,
                            right_hand_expression: Box::new(right_hand_expression),
                        });
                    }
                    (returned_iterator, returned_syntax_parsing_errors, None) => {
                        syntax_parsing_errors = returned_syntax_parsing_errors;
                        syntax_parsing_errors.push(format!(
                            "Syntax error : No right hand expression to prefix {:?}.",
                            token
                        ));
                        return (returned_iterator, syntax_parsing_errors, None);
                    }
                }
            }
            Token::TRUE | Token::FALSE => {
                debug!("Found an boolean expression.");
                expression = Some(Expression::BOOLEAN {
                    boolean_token: iterator.next().unwrap().clone(),
                });
            }
            Token::OPENING_ROUND_BRACKET => {
                debug!("Found a grouped expression.");
                match grouped_expression::parse_grouped_expression(iterator, syntax_parsing_errors)
                {
                    (
                        returned_iterator,
                        returned_syntax_parsing_errors,
                        Some(grouped_expression),
                    ) => {
                        iterator = returned_iterator;
                        syntax_parsing_errors = returned_syntax_parsing_errors;
                        expression = Some(grouped_expression);
                    }
                    (returned_iterator, returned_syntax_parsing_errors, None) => {
                        error!("Error parsing grouped expression, returning None.");
                        return (returned_iterator, returned_syntax_parsing_errors, None);
                    }
                }
            }
            Token::IF => {
                debug!("Found an if expression.");
                match if_expression::parse_if_expression(iterator, syntax_parsing_errors) {
                    (returned_iterator, returned_syntax_parsing_errors, Some(if_expression)) => {
                        iterator = returned_iterator;
                        syntax_parsing_errors = returned_syntax_parsing_errors;
                        expression = Some(if_expression);
                    }
                    (returned_iterator, returned_syntax_parsing_errors, None) => {
                        error!("Error parsing if expression, returning None.");
                        return (returned_iterator, returned_syntax_parsing_errors, None);
                    }
                }
            }
            Token::FUNCTION => {
                debug!("Found a function expression.");
                match function_expression::parse_function_expression(
                    iterator,
                    syntax_parsing_errors,
                ) {
                    (
                        returned_iterator,
                        returned_syntax_parsing_errors,
                        Some(function_expression),
                    ) => {
                        iterator = returned_iterator;
                        syntax_parsing_errors = returned_syntax_parsing_errors;
                        expression = Some(function_expression);
                    }
                    (returned_iterator, returned_syntax_parsing_errors, None) => {
                        error!("Error parsing function expression, returning None.");
                        return (returned_iterator, returned_syntax_parsing_errors, None);
                    }
                }
            }
            _ => {
                syntax_parsing_errors.push(format!(
                    "Syntax error : Do not know how to parse {:?} as an expression.",
                    token
                ));
                iterator.next();
                return (iterator, syntax_parsing_errors, None);
            }
        },
        None => {
            return (iterator, syntax_parsing_errors, None);
        }
    }

    return pratt_parsing::pratt_parsing(
        iterator,
        syntax_parsing_errors,
        expression,
        expression_precedence,
    );
}

#[cfg(test)]
mod tests;
