use crate::lexical_analysis::Token;
use crate::syntax_analysis::ExpressionPrecedence;
use crate::syntax_analysis::Expression;
use crate::syntax_analysis::SyntaxAnalysis;

impl SyntaxAnalysis<'_> {
    pub(crate) fn parse_grouped_expression(&mut self) -> anyhow::Result<Expression> {
        debug!("Parsing a grouped expression.");

        assert_token!(
            self,
            Token::OpeningRoundBracket,
            "A grouped expression must start with a OpeningRoundBracket token."
        );
        let grouped_expression = self.get_expression(ExpressionPrecedence::Lowest)?;
        assert_token!(
            self,
            Token::ClosingRoundBracket,
            "A grouped expression must end with a ClosingRoundBracket token."
        );

        Ok(grouped_expression)
    }
}
