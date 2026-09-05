use log::{debug, trace};

use crate::lexical_analysis::Token;
use crate::syntax_analysis::SyntaxAnalysis;
use crate::syntax_analysis::expression_precedence::ExpressionPrecedence;
use crate::syntax_analysis::{Statement, SyntaxTreeNode};

impl SyntaxAnalysis<'_> {
    pub(super) fn parse_return_statement(&mut self) -> anyhow::Result<SyntaxTreeNode> {
        debug!("Parsing a return statement.");

        assert_token!(
            self,
            Token::Return,
            "A return statement must start with Return token."
        );
        let expression = self.get_expression(ExpressionPrecedence::Lowest)?;
        semicolon!(self);

        Ok(SyntaxTreeNode::Statement {
            statement: Statement::Return {
                expression: Box::new(expression),
            },
        })
    }

    pub(super) fn parse_let_statement(&mut self) -> anyhow::Result<SyntaxTreeNode> {
        debug!("Parsing a let statement.");

        assert_token!(
            self,
            Token::Let,
            "A let statement must start with Let token."
        );
        let identifier = match self.tokens.next() {
            Some(Token::Identifier { literal }) => literal,
            _ => {
                anyhow::bail!(
                    "A let statement must have a variable identifier after the Let token."
                );
            }
        };
        assert_token!(
            self,
            Token::Assign,
            "A let statement must have an assignment operator after the Identifier token."
        );
        let expression = self.get_expression(ExpressionPrecedence::Lowest)?;
        semicolon!(self);

        Ok(SyntaxTreeNode::Statement {
            statement: Statement::Let {
                identifier: identifier.to_string(),
                expression: Box::new(expression),
            },
        })
    }
}
