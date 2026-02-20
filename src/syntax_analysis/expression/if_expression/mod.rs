use crate::lexical_analysis::model::token::Token;
use crate::syntax_analysis::model::expression_precedence::ExpressionPrecedence;
use crate::syntax_analysis::model::syntax_tree_node::Expression;
use crate::syntax_analysis::SyntaxAnalysis;

impl SyntaxAnalysis<'_> {
    pub(crate) fn parse_if_expression(&mut self) -> anyhow::Result<Expression> {
        debug!("Parsing a if expression.");

        // parse if expression
        assert_token!(self, Token::If, "A if expression must start with If token.");
        assert_token!(
            self,
            Token::OpeningRoundBracket,
            "A if expression must have a OpeningRoundBracket token after the If token."
        );
        let condition = self.get_expression(ExpressionPrecedence::Lowest)?;
        assert_token!(
            self,
            Token::ClosingRoundBracket,
            "A if expression must have a ClosingRoundBracket token after the conditional."
        );
        let consequence = self.parse_block()?;
        let mut alternative = None;

        if let Some(token) = self.tokens.peek() {
            if **token == Token::Else {
                // Consume else.
                self.tokens.next();
                alternative = Some(self.parse_block()?);
            }
        }

        Ok(Expression::If {
            condition: Box::new(condition),
            consequence: Box::new(consequence),
            alternative: Box::new(alternative),
        })
    }
}
