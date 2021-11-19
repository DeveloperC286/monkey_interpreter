use model::abstract_syntax_tree::syntax_tree_node::SyntaxTreeNode;
use model::abstract_syntax_tree::AbstractSyntaxTree;

use crate::lexical_analysis::model::token::Token;

#[macro_use]
mod macros;

pub(crate) mod model;

#[cfg(test)]
#[macro_use]
mod tests;

mod expression;
mod statement;

use std::iter::Peekable;
use std::slice::Iter;

pub(crate) struct SyntaxAnalysis<'a> {
    tokens: Peekable<Iter<'a, Token>>,
    syntax_parsing_errors: Vec<String>,
}

impl<'a> SyntaxAnalysis<'a> {
    pub(crate) fn from(tokens: Vec<Token>) -> AbstractSyntaxTree {
        let mut syntax_analysis = SyntaxAnalysis {
            tokens: tokens.iter().peekable(),
            syntax_parsing_errors: vec![],
        };

        syntax_analysis.get_abstract_syntax_tree()
    }

    pub(crate) fn get_abstract_syntax_tree(&mut self) -> AbstractSyntaxTree {
        let mut abstract_syntax_tree: Vec<SyntaxTreeNode> = vec![];

        while let Some(token) = self.tokens.peek() {
            match token {
                Token::EndOfFile => break,
                _ => {
                    if let Some(syntax_tree_node) = self.get_next_syntax_tree_node() {
                        abstract_syntax_tree.push(syntax_tree_node)
                    }
                }
            }
        }

        AbstractSyntaxTree {
            abstract_syntax_tree,
            syntax_parsing_errors: self.syntax_parsing_errors.clone(),
        }
    }

    fn get_next_syntax_tree_node(&mut self) -> Option<SyntaxTreeNode> {
        debug!("Parsing next SyntaxTreeNode.");
        match self.tokens.peek() {
            Some(token) => match token {
                Token::Let => self.parse_let_statement(),
                Token::Return => self.parse_return_statement(),
                _ => self.get_expression_node(),
            },
            None => None,
        }
    }
}
