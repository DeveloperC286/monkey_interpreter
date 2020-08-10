use std::iter::Peekable;
use std::slice::Iter;

use abstract_syntax_tree::syntax_tree_node::SyntaxTreeNode;
use abstract_syntax_tree::AbstractSyntaxTree;

use crate::lexical_analysis::token::Token;

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

    while let Some(token) = iterator.peek() {
        match token {
            Token::EOF => break,
            _ => {
                let (returned_iterator, returned_syntax_parsing_errors, syntax_tree_node) =
                    get_next_syntax_tree_node(iterator, syntax_parsing_errors);
                iterator = returned_iterator;
                syntax_parsing_errors = returned_syntax_parsing_errors;
                if let Some(syntax_tree_node) = syntax_tree_node {
                    abstract_syntax_tree.push(syntax_tree_node)
                }
            }
        }
    }

    AbstractSyntaxTree {
        abstract_syntax_tree,
        syntax_parsing_errors,
    }
}

fn get_next_syntax_tree_node(
    mut iterator: Peekable<Iter<Token>>,
    syntax_parsing_errors: Vec<String>,
) -> (Peekable<Iter<Token>>, Vec<String>, Option<SyntaxTreeNode>) {
    debug!("Parsing next SyntaxTreeNode.");
    match iterator.peek() {
        Some(token) => match token {
            Token::LET => statements::parse_let_statement(iterator, syntax_parsing_errors),
            Token::RETURN => statements::parse_return_statement(iterator, syntax_parsing_errors),
            _ => expressions::get_expression_node(iterator, syntax_parsing_errors),
        },
        None => (iterator, syntax_parsing_errors, None),
    }
}

#[cfg(test)]
mod tests;
