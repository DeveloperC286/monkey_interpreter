use crate::lexical_analysis::model::token::Token;
use crate::syntax_analysis::model::syntax_tree_node::InfixOperator;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub(crate) enum ExpressionPrecedence {
    Lowest,
    Equals,
    LesserOrGreater,
    Plus,
    Multiply,
    Prefix,
    Call,
}

pub(crate) fn get_current_expression_precedence(token: &Token) -> ExpressionPrecedence {
    match token {
        Token::Equals => ExpressionPrecedence::Equals,
        Token::NotEquals => ExpressionPrecedence::Equals,
        Token::LesserThan => ExpressionPrecedence::LesserOrGreater,
        Token::GreaterThan => ExpressionPrecedence::LesserOrGreater,
        Token::Plus => ExpressionPrecedence::Plus,
        Token::Minus => ExpressionPrecedence::Plus,
        Token::Multiply => ExpressionPrecedence::Multiply,
        Token::Divide => ExpressionPrecedence::Multiply,
        Token::OpeningRoundBracket => ExpressionPrecedence::Call,
        _ => {
            trace!("Could not find precedence for Token::{token:?} so returning LOWEST.");
            ExpressionPrecedence::Lowest
        }
    }
}

pub(crate) fn get_infix_operator_precedence(operator: &InfixOperator) -> ExpressionPrecedence {
    match operator {
        InfixOperator::Equals | InfixOperator::NotEquals => ExpressionPrecedence::Equals,
        InfixOperator::LesserThan | InfixOperator::GreaterThan => {
            ExpressionPrecedence::LesserOrGreater
        }
        InfixOperator::Plus | InfixOperator::Minus => ExpressionPrecedence::Plus,
        InfixOperator::Multiply | InfixOperator::Divide => ExpressionPrecedence::Multiply,
    }
}
