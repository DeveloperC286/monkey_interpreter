use crate::lexical_analysis::model::token::Token;
use crate::syntax_analysis::model::syntax_error::SyntaxError;
use crate::syntax_analysis::model::syntax_tree_node::SyntaxTreeNode;

#[macro_use]
mod macros;

pub(crate) mod model;

mod expression;
mod statement;

use std::iter::Peekable;
use std::slice::Iter;

pub(crate) struct SyntaxAnalysis<'a> {
    tokens: Peekable<Iter<'a, Token>>,
}

impl<'a> SyntaxAnalysis<'a> {
    pub(crate) fn from(tokens: Vec<Token>) -> Result<Vec<SyntaxTreeNode>, SyntaxError> {
        let mut syntax_analysis = SyntaxAnalysis {
            tokens: tokens.iter().peekable(),
        };

        syntax_analysis.get_abstract_syntax_tree()
    }

    pub(crate) fn get_abstract_syntax_tree(&mut self) -> Result<Vec<SyntaxTreeNode>, SyntaxError> {
        let mut abstract_syntax_tree: Vec<SyntaxTreeNode> = vec![];

        while self.tokens.peek().is_some() {
            let syntax_tree_node = self.get_next_syntax_tree_node()?;
            abstract_syntax_tree.push(syntax_tree_node)
        }

        Ok(abstract_syntax_tree)
    }

    fn get_next_syntax_tree_node(&mut self) -> Result<SyntaxTreeNode, SyntaxError> {
        debug!("Parsing next SyntaxTreeNode.");

        match self.tokens.peek() {
            None => Err(SyntaxError::NoTokenToParse),
            Some(Token::Let) => self.parse_let_statement(),
            Some(Token::Return) => self.parse_return_statement(),
            Some(_) => self.get_expression_node(),
        }
    }
}
