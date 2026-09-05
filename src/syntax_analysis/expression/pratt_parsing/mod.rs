use crate::lexical_analysis::Token;
use crate::syntax_analysis::Expression;
use crate::syntax_analysis::SyntaxAnalysis;
use crate::syntax_analysis::expression_precedence::{
    ExpressionPrecedence, get_current_expression_precedence,
};

mod call_expression;
mod infix_expression;

impl SyntaxAnalysis<'_> {
    pub(super) fn pratt_parsing(
        &mut self,
        mut expression: Expression,
        minimum_expression_precedence: ExpressionPrecedence,
    ) -> anyhow::Result<Expression> {
        while let Some(token) = self.tokens.peek() {
            if **token == Token::SemiColon {
                break;
            }

            let next_expression_precedence = get_current_expression_precedence(token);

            if minimum_expression_precedence >= next_expression_precedence {
                break;
            }

            match token {
                Token::Plus
                | Token::Minus
                | Token::Divide
                | Token::Multiply
                | Token::Equals
                | Token::NotEquals
                | Token::LesserThan
                | Token::GreaterThan => {
                    expression = self.parse_infix_expression(expression)?;
                }
                Token::OpeningRoundBracket => {
                    expression = self.parse_call_expression(expression)?;
                }
                _ => {
                    break;
                }
            }
        }

        Ok(expression)
    }
}
