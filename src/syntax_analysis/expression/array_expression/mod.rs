use crate::lexical_analysis::model::token::Token;
use crate::syntax_analysis::expression::ExpressionPrecedence;
use crate::syntax_analysis::model::syntax_error::SyntaxError;
use crate::syntax_analysis::model::syntax_tree_node::Expression;
use crate::syntax_analysis::SyntaxAnalysis;

impl<'a> SyntaxAnalysis<'a> {
    pub(crate) fn parse_array_expression(&mut self) -> Result<Expression, SyntaxError> {
        debug!("Parsing a array expression.");

        assert_token!(
            self,
            Token::OpeningSquareBracket,
            Err(SyntaxError::MissingArrayExpressionOpeningSquareBracket)
        );

        let mut elements = vec![];

        if let Some(token) = self.tokens.peek() {
            if **token != Token::ClosingSquareBracket {
                loop {
                    elements.push(self.get_expression(ExpressionPrecedence::Lowest)?);
                    match self.tokens.peek() {
                        Some(token) => match token {
                            Token::ClosingSquareBracket => break,
                            Token::Comma => {
                                self.tokens.next();
                            }
                            _ => {
                                return Err(SyntaxError::ArrayExpressionElementsNotCommaSeperated);
                            }
                        },
                        None => {
                            return Err(SyntaxError::ArrayExpressionElementsEndedAbruptly);
                        }
                    }
                }
            }
        }

        assert_token!(
            self,
            Token::ClosingSquareBracket,
            Err(SyntaxError::MissingArrayExpressionClosingSquareBracket)
        );
        Ok(Expression::Array { elements })
    }
}
