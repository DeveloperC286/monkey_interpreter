use crate::lexical_analysis::model::token::Token;
use crate::syntax_analysis::SyntaxAnalysis;
use crate::syntax_analysis::model::expression_precedence::{
    ExpressionPrecedence, get_current_expression_precedence,
};
use crate::syntax_analysis::model::syntax_tree_node::Expression;

mod call_expression;
mod infix_expression;

impl SyntaxAnalysis<'_> {
    pub(crate) fn pratt_parsing(
        &mut self,
        mut expression: Expression,
        minimum_expression_precedence: ExpressionPrecedence,
    ) -> anyhow::Result<Expression> {
        while let Some(positioned_token) = self.tokens.peek() {
            if positioned_token.token == Token::SemiColon {
                break;
            }

            let next_expression_precedence =
                get_current_expression_precedence(&positioned_token.token);

            if minimum_expression_precedence >= next_expression_precedence {
                break;
            }

            match positioned_token.token {
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
