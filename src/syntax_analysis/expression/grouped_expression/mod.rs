use crate::lexical_analysis::model::token::Token;
use crate::syntax_analysis::model::expression_precedence::ExpressionPrecedence;
use crate::syntax_analysis::model::syntax_error::SyntaxError;
use crate::syntax_analysis::model::syntax_tree_node::Expression;
use crate::syntax_analysis::SyntaxAnalysis;

impl SyntaxAnalysis<'_> {
    pub(crate) fn parse_grouped_expression(&mut self) -> Result<Expression, SyntaxError> {
        debug!("Parsing a grouped expression.");

        assert_token!(
            self,
            Token::OpeningRoundBracket,
            Err(SyntaxError::MissingGroupedOpeningRoundBracket)
        );
        let grouped_expression = self.get_expression(ExpressionPrecedence::Lowest)?;
        assert_token!(
            self,
            Token::ClosingRoundBracket,
            Err(SyntaxError::MissingGroupedClosingRoundBracket)
        );

        Ok(grouped_expression)
    }
}
