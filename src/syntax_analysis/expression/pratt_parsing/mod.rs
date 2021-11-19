use std::cmp::Ordering;

use crate::lexical_analysis::model::token::Token;
use crate::syntax_analysis::model::abstract_syntax_tree::syntax_tree_node::Expression;
use crate::syntax_analysis::model::expression_precedence::ExpressionPrecedence;
use crate::syntax_analysis::SyntaxAnalysis;

mod call_expression;
mod infix_expression;

impl<'a> SyntaxAnalysis<'a> {
    pub(crate) fn pratt_parsing(
        &mut self,
        mut expression: Option<Expression>,
        expression_precedence: ExpressionPrecedence,
    ) -> Option<Expression> {
        while let Some(token) = self.tokens.peek() {
            if **token == Token::SemiColon {
                break;
            }

            //if expression_precedence.
            let expression_precedence_comparison = expression_precedence.partial_cmp(
            &crate::syntax_analysis::model::expression_precedence::get_current_expression_precedence(
                token,
            ),
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
                    let returned_expression = self.parse_infix_expression(expression.unwrap());
                    expression = returned_expression;
                }
                Token::OpeningRoundBracket => {
                    let returned_expression = self.parse_call_expression(expression.unwrap());
                    expression = returned_expression;
                }
                _ => {
                    break;
                }
            }
        }

        expression
    }
}
