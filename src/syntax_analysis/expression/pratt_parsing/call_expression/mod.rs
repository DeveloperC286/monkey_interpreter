use crate::lexical_analysis::model::token::Token;
use crate::syntax_analysis::model::expression_precedence::ExpressionPrecedence;
use crate::syntax_analysis::model::syntax_error::SyntaxError;
use crate::syntax_analysis::model::syntax_tree_node::Expression;
use crate::syntax_analysis::SyntaxAnalysis;

impl<'a> SyntaxAnalysis<'a> {
    pub(crate) fn parse_call_expression(
        &mut self,
        function: Expression,
    ) -> Result<Expression, SyntaxError> {
        debug!("Parsing a call expression.");

        // check call expression was correctly called by an identifier on inlined function.
        match &function {
            Expression::Identifier { identifier: _ } => {}
            Expression::Function {
                parameters: _,
                block: _,
            } => {}
            _ => {
                return Err(SyntaxError::CallExpressionNotIdentifierOrFunction);
            }
        }

        // parse call expression
        let arguments = self.parse_arguments()?;

        Ok(Expression::Call {
            function: Box::new(function),
            arguments,
        })
    }

    fn parse_arguments(&mut self) -> Result<Vec<Expression>, SyntaxError> {
        debug!("Parsing arguments.");

        assert_token!(
            self,
            Token::OpeningRoundBracket,
            Err(SyntaxError::MissingCallExpressionOpeningRoundBracket)
        );
        let mut arguments = vec![];

        if let Some(token) = self.tokens.peek() {
            if **token != Token::ClosingRoundBracket {
                loop {
                    let expression = self.get_expression(ExpressionPrecedence::Lowest)?;
                    arguments.push(expression);

                    match self.tokens.peek() {
                        Some(token) => match token {
                            Token::ClosingRoundBracket => break,
                            Token::Comma => {
                                self.tokens.next();
                            }
                            _ => {
                                return Err(SyntaxError::CallExpressionParameterNotCommaSeperated);
                            }
                        },
                        None => {
                            return Err(SyntaxError::CallExpressionParametersEndedAbruptly);
                        }
                    }
                }
            }
        }

        assert_token!(
            self,
            Token::ClosingRoundBracket,
            Err(SyntaxError::MissingCallExpressionClosingRoundBracket)
        );
        Ok(arguments)
    }
}

#[cfg(test)]
mod tests;
