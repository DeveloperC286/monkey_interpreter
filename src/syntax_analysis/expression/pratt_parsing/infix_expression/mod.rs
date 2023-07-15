use crate::lexical_analysis::model::token::Token;
use crate::syntax_analysis::model::syntax_error::SyntaxError;
use crate::syntax_analysis::model::syntax_tree_node::Expression;
use crate::syntax_analysis::SyntaxAnalysis;

impl<'a> SyntaxAnalysis<'a> {
    pub(crate) fn parse_infix_expression(
        &mut self,
        left_hand: Expression,
    ) -> Result<Expression, SyntaxError> {
        debug!("Parsing a infix expression.");

        let operator = self.tokens.next().ok_or(SyntaxError::NoTokenToParse)?;

        let precedence =
            crate::syntax_analysis::model::expression_precedence::get_current_expression_precedence(
                operator,
            );
        match operator {
            Token::Plus => {
                self.get_expression(precedence)
                    .map(|right_hand| Expression::PlusInfix {
                        left_hand: Box::new(left_hand),
                        right_hand: Box::new(right_hand),
                    })
            }
            Token::Minus => {
                self.get_expression(precedence)
                    .map(|right_hand| Expression::MinusInfix {
                        left_hand: Box::new(left_hand),
                        right_hand: Box::new(right_hand),
                    })
            }
            _ => self
                .get_expression(precedence)
                .map(|right_hand| Expression::Infix {
                    left_hand: Box::new(left_hand),
                    operator: operator.clone(),
                    right_hand: Box::new(right_hand),
                }),
        }
    }
}
