use crate::lexical_analysis::model::token::Token;
use crate::syntax_analysis::model::abstract_syntax_tree::syntax_tree_node::Expression;
use crate::syntax_analysis::model::expression_precedence::ExpressionPrecedence;
use crate::syntax_analysis::SyntaxAnalysis;

impl<'a> SyntaxAnalysis<'a> {
    pub(crate) fn parse_grouped_expression(&mut self) -> Option<Expression> {
        debug!("Parsing a grouped expression.");

        assert_token!(self, Token::OpeningRoundBracket, None);
        let grouped_expression = self.get_expression(ExpressionPrecedence::Lowest);
        assert_token!(self, Token::ClosingRoundBracket, None);

        grouped_expression
    }
}

#[cfg(test)]
mod tests;
