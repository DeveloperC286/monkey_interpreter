use crate::lexical_analysis::model::token::Token;
use crate::syntax_analysis::model::abstract_syntax_tree::syntax_tree_node::Expression;
use crate::syntax_analysis::model::expression_precedence::ExpressionPrecedence;
use crate::syntax_analysis::SyntaxAnalysis;

impl<'a> SyntaxAnalysis<'a> {
    pub(crate) fn parse_if_expression(&mut self) -> Option<Expression> {
        debug!("Parsing a if expression.");

        // parse if expression
        assert_token!(self, Token::If, None);
        assert_token!(self, Token::OpeningRoundBracket, None);
        let condition_option = self.get_expression(ExpressionPrecedence::Lowest);
        assert_token!(self, Token::ClosingRoundBracket, None);
        let consequence_option = self.parse_block();
        let mut alternative = None;

        if let Some(token) = self.tokens.peek() {
            if **token == Token::Else {
                assert_token!(self, Token::Else, None);
                alternative = self.parse_block();
            }
        }

        // check if expression was parsed correctly
        let condition = match condition_option {
            Some(condition) => condition,
            None => {
                return None;
            }
        };

        let consequence = match consequence_option {
            Some(consequence) => consequence,
            None => {
                return None;
            }
        };

        Some(Expression::If {
            condition: Box::new(condition),
            consequence: Box::new(consequence),
            alternative: Box::new(alternative),
        })
    }
}

#[cfg(test)]
mod tests;
