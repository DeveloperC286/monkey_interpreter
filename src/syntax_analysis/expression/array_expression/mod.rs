use crate::lexical_analysis::model::token::Token;
use crate::syntax_analysis::expression::ExpressionPrecedence;
use crate::syntax_analysis::model::syntax_tree_node::Expression;
use crate::syntax_analysis::SyntaxAnalysis;

impl<'a> SyntaxAnalysis<'a> {
    pub(crate) fn parse_array_expression(&mut self) -> anyhow::Result<Expression> {
        debug!("Parsing a array expression.");

        assert_token!(
            self,
            Token::OpeningSquareBracket,
            "A array expression must start with a OpeningSquareBracket token."
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
                                anyhow::bail!("Array expression elements must be comma separated.");
                            }
                        },
                        None => {
                            anyhow::bail!("ArrayExpressionElementsEndedAbruptly.");
                        }
                    }
                }
            }
        }

        assert_token!(
            self,
            Token::ClosingSquareBracket,
            "A array expression must end with a ClosingSquareBracket token."
        );
        Ok(Expression::Array { elements })
    }
}
