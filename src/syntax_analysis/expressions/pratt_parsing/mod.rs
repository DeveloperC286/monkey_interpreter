use std::cmp::Ordering;
use std::iter::Peekable;
use std::slice::Iter;

use crate::lexical_analysis::token::Token;
use crate::syntax_analysis::abstract_syntax_tree::syntax_tree_node::{
    Expression, ExpressionPrecedence,
};
use crate::syntax_analysis::expression_precedence;

mod call_expression;
mod infix_expression;

pub fn pratt_parsing(
    mut iterator: Peekable<Iter<Token>>,
    mut syntax_parsing_errors: Vec<String>,
    mut expression: Option<Expression>,
    expression_precedence: ExpressionPrecedence,
) -> (Peekable<Iter<Token>>, Vec<String>, Option<Expression>) {
    while let Some(token) = iterator.peek() {
        if **token == Token::SEMI_COLON {
            break;
        }

        //if expression_precedence.
        let expression_precedence_comparison = expression_precedence.partial_cmp(
            &expression_precedence::get_current_expression_precedence(&token),
        );
        if expression_precedence_comparison != Some(Ordering::Less) {
            break;
        }

        match token {
            Token::PLUS
            | Token::MINUS
            | Token::DIVIDE
            | Token::MULTIPLY
            | Token::EQUALS
            | Token::NOT_EQUALS
            | Token::LESSER_THAN
            | Token::GREATER_THAN => {
                let (returned_iterator, returned_syntax_parsing_errors, returned_expression) =
                    infix_expression::parse_infix_expression(
                        iterator,
                        syntax_parsing_errors,
                        expression.unwrap(),
                    );
                iterator = returned_iterator;
                syntax_parsing_errors = returned_syntax_parsing_errors;
                expression = returned_expression;
            }
            Token::OPENING_ROUND_BRACKET => {
                let (returned_iterator, returned_syntax_parsing_errors, returned_expression) =
                    call_expression::parse_call_expression(
                        iterator,
                        syntax_parsing_errors,
                        expression.unwrap(),
                    );
                iterator = returned_iterator;
                syntax_parsing_errors = returned_syntax_parsing_errors;
                expression = returned_expression;
            }
            _ => {
                break;
            }
        }
    }

    (iterator, syntax_parsing_errors, expression)
}
