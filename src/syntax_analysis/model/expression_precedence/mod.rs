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
        Token::Equals => ExpressionPrecedence::EQUALS,
        Token::NotEquals => ExpressionPrecedence::EQUALS,
        Token::LesserThan => ExpressionPrecedence::LESSER_OR_GREATER,
        Token::GreaterThan => ExpressionPrecedence::LESSER_OR_GREATER,
        Token::Plus => ExpressionPrecedence::PLUS,
        Token::Minus => ExpressionPrecedence::PLUS,
        Token::Multiply => ExpressionPrecedence::MULTIPLY,
        Token::Divide => ExpressionPrecedence::MULTIPLY,
        Token::OpeningRoundBracket => ExpressionPrecedence::CALL,
        _ => {
            trace!(
                "Could not find precedence for Token::{:?} so returning LOWEST.",
                token
            );
            ExpressionPrecedence::LOWEST
        }
    }
}
