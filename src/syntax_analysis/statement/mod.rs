use crate::lexical_analysis::model::token::Token;
use crate::syntax_analysis::model::expression_precedence::ExpressionPrecedence;
use crate::syntax_analysis::model::syntax_error::SyntaxError;
use crate::syntax_analysis::model::syntax_tree_node::{Statement, SyntaxTreeNode};
use crate::syntax_analysis::SyntaxAnalysis;

impl SyntaxAnalysis<'_> {
    pub(crate) fn parse_return_statement(&mut self) -> Result<SyntaxTreeNode, SyntaxError> {
        debug!("Parsing a return statement.");

        assert_token!(self, Token::Return, Err(SyntaxError::MissingReturn));
        let expression = self.get_expression(ExpressionPrecedence::Lowest)?;
        semicolon!(self);

        Ok(SyntaxTreeNode::Statement {
            statement: Statement::Return {
                expression: Box::new(expression),
            },
        })
    }

    pub(crate) fn parse_let_statement(&mut self) -> Result<SyntaxTreeNode, SyntaxError> {
        debug!("Parsing a let statement.");

        assert_token!(self, Token::Let, Err(SyntaxError::MissingLet));
        let identifier = match self.tokens.next() {
            Some(Token::Identifier { literal }) => literal,
            _ => {
                return Err(SyntaxError::MissingLetIdentifier);
            }
        };
        assert_token!(self, Token::Assign, Err(SyntaxError::MissingLetAssignment));
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
