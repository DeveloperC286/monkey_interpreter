use log::debug;

use crate::lexical_analysis::Token;

#[macro_use]
mod macros;

mod expression;
mod expression_precedence;
mod statement;
mod syntax_tree_node;

pub use syntax_tree_node::{Block, Expression, InfixOperator, Statement, SyntaxTreeNode};

use std::iter::Peekable;
use std::slice::Iter;

pub(crate) struct SyntaxAnalysis<'a> {
    tokens: Peekable<Iter<'a, Token>>,
}

impl SyntaxAnalysis<'_> {
    pub(crate) fn from(tokens: Vec<Token>) -> anyhow::Result<Vec<SyntaxTreeNode>> {
        let mut syntax_analysis = SyntaxAnalysis {
            tokens: tokens.iter().peekable(),
        };

        syntax_analysis.get_abstract_syntax_tree()
    }

    pub(super) fn get_abstract_syntax_tree(&mut self) -> anyhow::Result<Vec<SyntaxTreeNode>> {
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
            None => anyhow::bail!("No token to parse."),
            Some(Token::Let) => self.parse_let_statement(),
            Some(Token::Return) => self.parse_return_statement(),
            Some(_) => self.get_expression_node(),
        }
    }
}
