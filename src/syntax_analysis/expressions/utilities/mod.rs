use std::iter::Peekable;
use std::slice::Iter;

use crate::lexical_analysis::token::{Token, TokenType};
use crate::syntax_analysis::abstract_syntax_tree::syntax_tree_node::Block;
use crate::syntax_analysis::get_next_syntax_tree_node;

pub fn parse_block(
    mut iterator: Peekable<Iter<Token>>,
    mut syntax_parsing_errors: Vec<String>,
) -> (Peekable<Iter<Token>>, Vec<String>, Option<Block>) {
    debug!("Parsing a block.");
    assert_token!(
        iterator,
        syntax_parsing_errors,
        TokenType::OPENING_CURLY_BRACKET,
        None
    );
    let mut blocks = vec![];

    loop {
        match iterator.peek() {
            Some(token) => match token.token_type {
                TokenType::CLOSING_CURLY_BRACKET | TokenType::EOF => break,
                _ => match get_next_syntax_tree_node(iterator, syntax_parsing_errors) {
                    (returned_iterator, returned_syntax_parsing_errors, Some(token)) => {
                        iterator = returned_iterator;
                        syntax_parsing_errors = returned_syntax_parsing_errors;
                        blocks.push(token)
                    }
                    (returned_iterator, returned_syntax_parsing_errors, None) => {
                        iterator = returned_iterator;
                        syntax_parsing_errors = returned_syntax_parsing_errors;
                    }
                },
            },
            None => break,
        }
    }

    assert_token!(
        iterator,
        syntax_parsing_errors,
        TokenType::CLOSING_CURLY_BRACKET,
        None
    );

    return (
        iterator,
        syntax_parsing_errors,
        Some(Block::BLOCK { blocks }),
    );
}
