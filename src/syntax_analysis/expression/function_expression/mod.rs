use crate::lexical_analysis::Token;
use crate::syntax_analysis::ExpressionPrecedence;
use crate::syntax_analysis::Expression;
use crate::syntax_analysis::SyntaxAnalysis;

impl SyntaxAnalysis<'_> {
    pub(crate) fn parse_function_expression(&mut self) -> anyhow::Result<Expression> {
        debug!("Parsing a function expression.");

        // parse function expression
        assert_token!(
            self,
            Token::Function,
            "A function expression must start with Function token."
        );
        let parameters = self.parse_parameters()?;

        // check function expression was parsed correctly
        let block = self.parse_block()?;

        Ok(Expression::Function {
            parameters,
            block: Box::new(block),
        })
    }

    fn parse_parameters(&mut self) -> anyhow::Result<Vec<String>> {
        debug!("Parsing parameters.");

        assert_token!(
            self,
            Token::OpeningRoundBracket,
            "A function expression must have a OpeningRoundBracket token after the Function token."
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
                            anyhow::bail!(
                                "Only allowed identifiers in function expression's parameters."
                            );
                        }
                    }

                    match self.tokens.peek() {
                        Some(token) => match token {
                            Token::ClosingRoundBracket => break,
                            Token::Comma => {
                                self.tokens.next();
                            }
                            _ => {
                                anyhow::bail!("Parameters must be comma seperated identifiers.");
                            }
                        },
                        None => {
                            anyhow::bail!("FunctionParametersEndedAbruptly.");
                        }
                    }
                }
            }
        }

        assert_token!(
            self,
            Token::ClosingRoundBracket,
            "A function expression must have a ClosingRoundBracket token after the parameters."
        );
        Ok(parameters)
    }
}
