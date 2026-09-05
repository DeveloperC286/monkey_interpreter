use log::debug;

use crate::lexical_analysis::model::token::{PositionedToken, Token};
use crate::syntax_analysis::model::syntax_tree_node::SyntaxTreeNode;

#[macro_use]
mod macros;

pub(crate) mod model;

mod expression;
mod statement;

use std::iter::Peekable;
use std::slice::Iter;

pub(crate) struct SyntaxAnalysis<'a> {
    tokens: Peekable<Iter<'a, PositionedToken>>,
}

impl SyntaxAnalysis<'_> {
    pub(crate) fn from(tokens: Vec<PositionedToken>) -> anyhow::Result<Vec<SyntaxTreeNode>> {
        let mut syntax_analysis = SyntaxAnalysis {
            tokens: tokens.iter().peekable(),
        };

        syntax_analysis.get_abstract_syntax_tree()
    }

    pub(crate) fn get_abstract_syntax_tree(&mut self) -> anyhow::Result<Vec<SyntaxTreeNode>> {
        let mut abstract_syntax_tree: Vec<SyntaxTreeNode> = vec![];

        while self.tokens.peek().is_some() {
            let syntax_tree_node = self.get_next_syntax_tree_node()?;
            abstract_syntax_tree.push(syntax_tree_node)
        }

        Ok(abstract_syntax_tree)
    }

    fn get_next_syntax_tree_node(&mut self) -> anyhow::Result<SyntaxTreeNode> {
        debug!("Parsing next SyntaxTreeNode.");

        match self.tokens.peek() {
            None => anyhow::bail!("No token to parse, reached the end of the code."),
            Some(PositionedToken {
                token: Token::Let, ..
            }) => self.parse_let_statement(),
            Some(PositionedToken {
                token: Token::Return,
                ..
            }) => self.parse_return_statement(),
            Some(_) => self.get_expression_node(),
        }
    }
}
