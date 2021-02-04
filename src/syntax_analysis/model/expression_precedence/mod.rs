use crate::lexical_analysis::model::token::Token;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
pub enum ExpressionPrecedence {
    LOWEST,
    EQUALS,
    LESSER_OR_GREATER,
    PLUS,
    MULTIPLY,
    PREFIX,
    CALL,
}

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
