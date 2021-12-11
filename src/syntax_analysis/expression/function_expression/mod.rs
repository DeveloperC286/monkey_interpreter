use crate::lexical_analysis::model::token::Token;
use crate::syntax_analysis::model::expression_precedence::ExpressionPrecedence;
use crate::syntax_analysis::model::syntax_error::SyntaxError;
use crate::syntax_analysis::model::syntax_tree_node::Expression;
use crate::syntax_analysis::SyntaxAnalysis;

impl<'a> SyntaxAnalysis<'a> {
    pub(crate) fn parse_function_expression(&mut self) -> Result<Expression, SyntaxError> {
        debug!("Parsing a function expression.");

        // parse function expression
        assert_token!(self, Token::Function, Err(SyntaxError::MissingFunction));
        let parameters = self.parse_parameters()?;

        // check function expression was parsed correctly
        let block = self.parse_block()?;

        Ok(Expression::Function {
            parameters,
            block: Box::new(block),
        })
    }

    fn parse_parameters(&mut self) -> Result<Vec<String>, SyntaxError> {
        debug!("Parsing parameters.");

        assert_token!(
            self,
            Token::OpeningRoundBracket,
            Err(SyntaxError::MissingFunctionOpeningRoundBracket)
        );
        let mut parameters = vec![];

        if let Some(token) = self.tokens.peek() {
            if **token != Token::ClosingRoundBracket {
                loop {
                    let expression = self.get_expression(ExpressionPrecedence::Lowest)?;
                    match expression {
                        Expression::Identifier { identifier } => {
                            parameters.push(identifier);
                        }
                        _ => {
                            return Err(SyntaxError::FunctionParameterNotIdentifier);
                        }
                    }

                    match self.tokens.peek() {
                        Some(token) => match token {
                            Token::ClosingRoundBracket => break,
                            Token::Comma => {
                                self.tokens.next();
                            }
                            _ => {
                                return Err(SyntaxError::FunctionParameterNotCommaSeperated);
                            }
                        },
                        None => {
                            return Err(SyntaxError::FunctionParametersEndedAbruptly);
                        }
                    }
                }
            }
        }

        assert_token!(
            self,
            Token::ClosingRoundBracket,
            Err(SyntaxError::MissingFunctionClosingRoundBracket)
        );
        Ok(parameters)
    }
}

#[cfg(test)]
mod tests;
