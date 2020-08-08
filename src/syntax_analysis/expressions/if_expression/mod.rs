use std::iter::Peekable;
use std::slice::Iter;

use crate::lexical_analysis::token::Token;
use crate::syntax_analysis::abstract_syntax_tree::syntax_tree_node::{
    Expression, ExpressionPrecedence,
};
use crate::syntax_analysis::expressions::get_expression;
use crate::syntax_analysis::expressions::utilities::parse_block;

pub fn parse_if_expression(
    mut iterator: Peekable<Iter<Token>>,
    mut syntax_parsing_errors: Vec<String>,
) -> (Peekable<Iter<Token>>, Vec<String>, Option<Expression>) {
    debug!("Parsing a if expression.");

    // parse if expression
    assert_token!(iterator, syntax_parsing_errors, Token::IF, None);
    assert_token!(
        iterator,
        syntax_parsing_errors,
        Token::OPENING_ROUND_BRACKET,
        None
    );
    let (returned_iterator, returned_syntax_parsing_errors, condition_option) = get_expression(
        iterator,
        syntax_parsing_errors,
        ExpressionPrecedence::LOWEST,
    );
    iterator = returned_iterator;
    syntax_parsing_errors = returned_syntax_parsing_errors;
    assert_token!(
        iterator,
        syntax_parsing_errors,
        Token::CLOSING_ROUND_BRACKET,
        None
    );
    let (returned_iterator, returned_syntax_parsing_errors, consequence_option) =
        parse_block(iterator, syntax_parsing_errors);
    iterator = returned_iterator;
    syntax_parsing_errors = returned_syntax_parsing_errors;
    let mut alternative = None;

    if let Some(token) = iterator.peek() {
        if **token == Token::ELSE {
            assert_token!(iterator, syntax_parsing_errors, Token::ELSE, None);
            let (returned_iterator, returned_syntax_parsing_errors, returned_alternative) =
                parse_block(iterator, syntax_parsing_errors);
            alternative = returned_alternative;
            iterator = returned_iterator;
            syntax_parsing_errors = returned_syntax_parsing_errors;
        }
    }

    // check if expression was parsed correctly
    let condition = match condition_option {
        Some(condition) => condition,
        None => {
            return (iterator, syntax_parsing_errors, None);
        }
    };

    let consequence = match consequence_option {
        Some(consequence) => consequence,
        None => {
            return (iterator, syntax_parsing_errors, None);
        }
    };

    (
        iterator,
        syntax_parsing_errors,
        Some(Expression::IF {
            condition: Box::new(condition),
            consequence: Box::new(consequence),
            alternative: Box::new(alternative),
        }),
    )
}

#[cfg(test)]
mod tests;
