use crate::lexical_analysis::Token;
use crate::syntax_analysis::ExpressionPrecedence;
use crate::syntax_analysis::Expression;
use crate::syntax_analysis::SyntaxAnalysis;

impl SyntaxAnalysis<'_> {
    pub(crate) fn parse_call_expression(
        &mut self,
        function: Expression,
    ) -> anyhow::Result<Expression> {
        debug!("Parsing a call expression.");

        // check call expression was correctly called by an identifier on inlined function.
        match &function {
            Expression::Identifier { identifier: _ } => {}
            Expression::Function {
                parameters: _,
                block: _,
            } => {}
            _ => {
                anyhow::bail!(
                    "A call expression is not calling either an identifier or an inlined function."
                );
            }
        }

        // parse call expression
        let arguments = self.parse_arguments()?;

        Ok(Expression::Call {
            function: Box::new(function),
            arguments,
        })
    }

    fn parse_arguments(&mut self) -> anyhow::Result<Vec<Expression>> {
        debug!("Parsing arguments.");

        assert_token!(
            self,
            Token::OpeningRoundBracket,
            "A call expression must have a OpeningRoundBracket token after the Function token."
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
                                anyhow::bail!("Parameters must be comma seperated identifiers.");
                            }
                        },
                        None => {
                            anyhow::bail!("CallExpressionParametersEndedAbruptly.");
                        }
                    }
                }
            }
        }

        assert_token!(
            self,
            Token::ClosingRoundBracket,
            "A call expression must have a ClosingRoundBracket token after the parameters."
        );
        Ok(arguments)
    }
}
