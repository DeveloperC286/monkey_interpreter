use std::cmp::Ordering;

use crate::lexical_analysis::model::token::Token;
use crate::syntax_analysis::model::expression_precedence::ExpressionPrecedence;
use crate::syntax_analysis::model::syntax_error::SyntaxError;
use crate::syntax_analysis::model::syntax_tree_node::Expression;
use crate::syntax_analysis::SyntaxAnalysis;

mod call_expression;
mod infix_expression;

impl SyntaxAnalysis<'_> {
    pub(crate) fn pratt_parsing(
        &mut self,
        mut expression: Expression,
        expression_precedence: ExpressionPrecedence,
    ) -> Result<Expression, SyntaxError> {
        while let Some(token) = self.tokens.peek() {
            if **token == Token::SemiColon {
                break;
            }

            let expression_precedence_comparison = expression_precedence.partial_cmp(
            &crate::syntax_analysis::model::expression_precedence::get_current_expression_precedence(
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
