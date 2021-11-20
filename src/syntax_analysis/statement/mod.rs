use crate::lexical_analysis::model::token::Token;
use crate::syntax_analysis::model::abstract_syntax_tree::syntax_tree_node::{
    Statement, SyntaxTreeNode,
};
use crate::syntax_analysis::model::expression_precedence::ExpressionPrecedence;
use crate::syntax_analysis::SyntaxAnalysis;

#[macro_use]
mod macros;

impl<'a> SyntaxAnalysis<'a> {
    pub(crate) fn parse_return_statement(&mut self) -> Option<SyntaxTreeNode> {
        debug!("Parsing a return statement.");

        assert_token!(self, Token::Return, None);
        let expression = consume_expression!(self);
        semicolon!(self);

        Some(SyntaxTreeNode::Statement {
            statement: Statement::Return {
                expression: Box::new(expression),
            },
        })
    }

    pub(crate) fn parse_let_statement(&mut self) -> Option<SyntaxTreeNode> {
        debug!("Parsing a let statement.");

        assert_token!(self, Token::Let, None);
        let identifier = match self.tokens.next() {
            Some(token) => match token {
                Token::Identifier { literal } => literal,
                _ => {
                    self.syntax_parsing_errors
                        .push("Syntax error : Expected a IDENTIFIER.".to_string());
                    return None;
                }
            },
            None => {
                return None;
            }
        };
        assert_token!(self, Token::Assign, None);
        let expression = consume_expression!(self);

        semicolon!(self);

        Some(SyntaxTreeNode::Statement {
            statement: Statement::Let {
                identifier: identifier.to_string(),
                expression: Box::new(expression),
            },
        })
    }
}

#[cfg(test)]
mod tests;
