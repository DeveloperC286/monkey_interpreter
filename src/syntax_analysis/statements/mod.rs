use std::iter::Peekable;
use std::slice::Iter;

use crate::lexical_analysis::token::Token;
use crate::syntax_analysis::abstract_syntax_tree::syntax_tree_node::{
    ExpressionPrecedence, Statement, SyntaxTreeNode,
};
use crate::syntax_analysis::expressions;

#[macro_use]
pub mod macros;

pub fn parse_return_statement(
    mut iterator: Peekable<Iter<Token>>,
    mut syntax_parsing_errors: Vec<String>,
) -> (Peekable<Iter<Token>>, Vec<String>, Option<SyntaxTreeNode>) {
    debug!("Parsing a return statement.");

    assert_token!(iterator, syntax_parsing_errors, Token::RETURN, None);
    let expression = consume_expression!(iterator, syntax_parsing_errors);
    semicolon!(iterator);

    return (
        iterator,
        syntax_parsing_errors,
        Some(SyntaxTreeNode::STATEMENT {
            statement: Statement::RETURN {
                expression: Box::new(expression),
            },
        }),
    );
}

pub fn parse_let_statement(
    mut iterator: Peekable<Iter<Token>>,
    mut syntax_parsing_errors: Vec<String>,
) -> (Peekable<Iter<Token>>, Vec<String>, Option<SyntaxTreeNode>) {
    debug!("Parsing a let statement.");

    assert_token!(iterator, syntax_parsing_errors, Token::LET, None);
    let identifier_token = match iterator.next() {
        Some(token) => match token {
            Token::IDENTIFIER { literal } => {
                token
            }
            _ => {
                syntax_parsing_errors.push(format!("Syntax error : Expected a IDENTIFIER.",));
                return (iterator, syntax_parsing_errors, None);
            }
        }
        None => {
            return (iterator, syntax_parsing_errors, None);
        }
    };
    assert_token!(iterator, syntax_parsing_errors, Token::ASSIGN, None);
    let expression = consume_expression!(iterator, syntax_parsing_errors);

    semicolon!(iterator);

    return (
        iterator,
        syntax_parsing_errors,
        Some(SyntaxTreeNode::STATEMENT {
            statement: Statement::LET {
                identifier_token: identifier_token.clone(),
                expression: Box::new(expression),
            },
        }),
    );
}

#[cfg(test)]
mod tests;
