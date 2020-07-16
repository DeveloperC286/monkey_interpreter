use std::iter::Peekable;
use std::slice::Iter;

use abstract_syntax_tree::syntax_tree_node::SyntaxTreeNode;
use abstract_syntax_tree::AbstractSyntaxTree;

use crate::lexical_analysis::token::{Token, TokenType};

#[macro_use]
pub mod macros;

pub mod abstract_syntax_tree;
pub mod expression_precedence;
pub mod expressions;
pub mod statements;

pub fn get_abstract_syntax_tree(tokens: Vec<Token>) -> AbstractSyntaxTree {
    let mut abstract_syntax_tree: Vec<SyntaxTreeNode> = vec![];
    let mut syntax_parsing_errors: Vec<String> = vec![];
    let mut iterator: Peekable<Iter<Token>> = tokens.iter().peekable();

    loop {
        match iterator.peek() {
            Some(token) => match token.token_type {
                TokenType::EOF => break,
                _ => {
                    let (returned_iterator, returned_syntax_parsing_errors, syntax_tree_node) =
                        get_next_syntax_tree_node(iterator, syntax_parsing_errors);
                    iterator = returned_iterator;
                    syntax_parsing_errors = returned_syntax_parsing_errors;
                    match syntax_tree_node {
                        Some(syntax_tree_node) => abstract_syntax_tree.push(syntax_tree_node),
                        None => {}
                    }
                }
            },
            None => {
                break;
            }
        }
    }

    return AbstractSyntaxTree {
        abstract_syntax_tree,
        syntax_parsing_errors,
    };
}

fn get_next_syntax_tree_node(
    mut iterator: Peekable<Iter<Token>>,
    mut syntax_parsing_errors: Vec<String>,
) -> (Peekable<Iter<Token>>, Vec<String>, Option<SyntaxTreeNode>) {
    debug!("Parsing next SyntaxTreeNode.");
    match iterator.peek() {
        Some(token) => match token.token_type {
            TokenType::LET => statements::parse_let_statement(iterator, syntax_parsing_errors),
            TokenType::RETURN => {
                statements::parse_return_statement(iterator, syntax_parsing_errors)
            }
            _ => expressions::parse_expression(iterator, syntax_parsing_errors),
        },
        None => (iterator, syntax_parsing_errors, None),
    }
}

#[cfg(test)]
mod tests;
