use std::iter::Peekable;
use std::slice::Iter;

use crate::lexical_analysis::token::Token;
use crate::syntax_analysis::abstract_syntax_tree::syntax_tree_node::Block;

pub fn parse_block(
    mut iterator: Peekable<Iter<Token>>,
    mut syntax_parsing_errors: Vec<String>,
) -> (Peekable<Iter<Token>>, Vec<String>, Option<Block>) {
    debug!("Parsing a block.");
    assert_token!(
        iterator,
        syntax_parsing_errors,
        Token::OPENING_CURLY_BRACKET,
        None
    );
    let mut blocks = vec![];

    while let Some(token) = iterator.peek() {
        match token {
            Token::CLOSING_CURLY_BRACKET | Token::EOF => break,
            _ => match crate::syntax_analysis::get_next_syntax_tree_node(iterator, syntax_parsing_errors) {
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
        }
    }

    assert_token!(
        iterator,
        syntax_parsing_errors,
        Token::CLOSING_CURLY_BRACKET,
        None
    );

    (
        iterator,
        syntax_parsing_errors,
        Some(Block { nodes: blocks }),
    )
}
