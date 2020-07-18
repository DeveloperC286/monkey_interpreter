use std::iter::Peekable;
use std::slice::Iter;

use crate::lexical_analysis::token::{Token, TokenType};
use crate::syntax_analysis::abstract_syntax_tree::syntax_tree_node::{
    ExpressionPrecedence, Statement, SyntaxTreeNode,
};
use crate::syntax_analysis::expressions;

pub fn parse_return_statement(
    mut iterator: Peekable<Iter<Token>>,
    mut syntax_parsing_errors: Vec<String>,
) -> (Peekable<Iter<Token>>, Vec<String>, Option<SyntaxTreeNode>) {
    debug!("Parsing a return statement.");

    assert_token!(iterator, syntax_parsing_errors, TokenType::RETURN, None);

    let expression = match expressions::get_expression(
        iterator,
        syntax_parsing_errors,
        ExpressionPrecedence::LOWEST,
    ) {
        (returned_iterator, returned_syntax_parsing_errors, Some(expression)) => {
            iterator = returned_iterator;
            syntax_parsing_errors = returned_syntax_parsing_errors;
            expression
        }
        (returned_iterator, returned_syntax_parsing_errors, None) => {
            //TODO syntax_error!(self, "".to_string());
            return (returned_iterator, returned_syntax_parsing_errors, None);
        }
    };

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

    assert_token!(iterator, syntax_parsing_errors, TokenType::LET, None);

    let identifier_token = iterator.peek().unwrap().clone();
    assert_token!(iterator, syntax_parsing_errors, TokenType::IDENTIFIER, None);

    assert_token!(iterator, syntax_parsing_errors, TokenType::ASSIGN, None);

    let expression = match expressions::get_expression(
        iterator,
        syntax_parsing_errors,
        ExpressionPrecedence::LOWEST,
    ) {
        (returned_iterator, returned_syntax_parsing_errors, Some(expression)) => {
            iterator = returned_iterator;
            syntax_parsing_errors = returned_syntax_parsing_errors;
            expression
        }
        (returned_iterator, returned_syntax_parsing_errors, None) => {
            //TODO syntax_error!(self, "".to_string());
            return (returned_iterator, returned_syntax_parsing_errors, None);
        }
    };

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
