use std::cmp::Ordering;

use crate::lexical_analysis::Token;
use crate::syntax_analysis::ExpressionPrecedence;
use crate::syntax_analysis::Expression;
use crate::syntax_analysis::SyntaxAnalysis;

mod call_expression;
mod infix_expression;

impl SyntaxAnalysis<'_> {
    pub(crate) fn pratt_parsing(
        &mut self,
        mut expression: Expression,
        expression_precedence: ExpressionPrecedence,
    ) -> anyhow::Result<Expression> {
        while let Some(token) = self.tokens.peek() {
            if **token == Token::SemiColon {
                break;
            }

            let expression_precedence_comparison = expression_precedence.partial_cmp(
            &crate::syntax_analysis::get_current_expression_precedence(
                token,
            )
        );

            if expression_precedence_comparison != Some(Ordering::Less) {
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
