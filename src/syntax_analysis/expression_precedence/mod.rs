use crate::lexical_analysis::model::token::Token;
use crate::syntax_analysis::abstract_syntax_tree::syntax_tree_node::ExpressionPrecedence;

pub fn get_current_expression_precedence(token: &Token) -> ExpressionPrecedence {
    match token {
        Token::EQUALS => ExpressionPrecedence::EQUALS,
        Token::NOT_EQUALS => ExpressionPrecedence::EQUALS,
        Token::LESSER_THAN => ExpressionPrecedence::LESSER_OR_GREATER,
        Token::GREATER_THAN => ExpressionPrecedence::LESSER_OR_GREATER,
        Token::PLUS => ExpressionPrecedence::PLUS,
        Token::MINUS => ExpressionPrecedence::PLUS,
        Token::MULTIPLY => ExpressionPrecedence::MULTIPLY,
        Token::DIVIDE => ExpressionPrecedence::MULTIPLY,
        Token::OPENING_ROUND_BRACKET => ExpressionPrecedence::CALL,
        _ => {
            trace!(
                "Could not find precedence for Token::{:?} so returning LOWEST.",
                token
            );
            ExpressionPrecedence::LOWEST
        }
    }
}
