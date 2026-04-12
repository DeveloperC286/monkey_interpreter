use crate::lexical_analysis::Token;
use crate::syntax_analysis::get_infix_operator_precedence;
use crate::syntax_analysis::{Expression, InfixOperator};
use crate::syntax_analysis::SyntaxAnalysis;

impl SyntaxAnalysis<'_> {
    pub(crate) fn parse_infix_expression(
        &mut self,
        left_hand: Expression,
    ) -> anyhow::Result<Expression> {
        debug!("Parsing a infix expression.");

        let token = self
            .tokens
            .next()
            .ok_or_else(|| anyhow::anyhow!("No token to parse."))?;

        let operator = match token {
            Token::Plus => InfixOperator::Plus,
            Token::Minus => InfixOperator::Minus,
            Token::Multiply => InfixOperator::Multiply,
            Token::Divide => InfixOperator::Divide,
            Token::Equals => InfixOperator::Equals,
            Token::NotEquals => InfixOperator::NotEquals,
            Token::LesserThan => InfixOperator::LesserThan,
            Token::GreaterThan => InfixOperator::GreaterThan,
            _ => anyhow::bail!("Unknown infix operator token {:?}.", token),
        };

        let precedence = get_infix_operator_precedence(&operator);

        self.get_expression(precedence)
            .map(|right_hand| Expression::Infix {
                left_hand: Box::new(left_hand),
                operator,
                right_hand: Box::new(right_hand),
            })
    }
}
