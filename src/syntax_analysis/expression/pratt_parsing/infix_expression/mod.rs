use crate::syntax_analysis::model::abstract_syntax_tree::syntax_tree_node::Expression;
use crate::syntax_analysis::SyntaxAnalysis;

impl<'a> SyntaxAnalysis<'a> {
    pub(crate) fn parse_infix_expression(&mut self, left_hand: Expression) -> Option<Expression> {
        debug!("Parsing a infix expression.");

        let operator_token = match self.tokens.next() {
            Some(token) => token,
            None => return None,
        };
        let precedence =
            crate::syntax_analysis::model::expression_precedence::get_current_expression_precedence(
                operator_token,
            );

        match self.get_expression(precedence) {
            Some(right_hand) => Some(Expression::Infix {
                left_hand: Box::new(left_hand),
                operator_token: operator_token.clone(),
                right_hand: Box::new(right_hand),
            }),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests;
