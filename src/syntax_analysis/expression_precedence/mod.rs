use std::collections::HashMap;

use crate::lexical_analysis::token::TokenType;
use crate::syntax_analysis::abstract_syntax_tree::syntax_tree_node::ExpressionPrecedence;

lazy_static! {
    static ref TOKEN_PRECEDENCES: HashMap<TokenType, ExpressionPrecedence> = {
        let mut m = HashMap::new();
        m.insert(TokenType::EQUALS, ExpressionPrecedence::EQUALS);
        m.insert(TokenType::NOT_EQUALS, ExpressionPrecedence::EQUALS);
        m.insert(
            TokenType::LESSER_THAN,
            ExpressionPrecedence::LESSER_OR_GREATER,
        );
        m.insert(
            TokenType::GREATER_THAN,
            ExpressionPrecedence::LESSER_OR_GREATER,
        );
        m.insert(TokenType::PLUS, ExpressionPrecedence::PLUS);
        m.insert(TokenType::MINUS, ExpressionPrecedence::PLUS);
        m.insert(TokenType::MULTIPLY, ExpressionPrecedence::MULTIPLY);
        m.insert(TokenType::DIVIDE, ExpressionPrecedence::MULTIPLY);
        m.insert(TokenType::OPENING_ROUND_BRACKET, ExpressionPrecedence::CALL);
        m
    };
}

pub fn get_current_expression_precedence(token_type: &TokenType) -> ExpressionPrecedence {
    match TOKEN_PRECEDENCES.get(token_type) {
        Some(expression_precedence) => {
            trace!(
                "Found precedence {:?} for TokenType::{:?}.",
                expression_precedence,
                token_type
            );
            expression_precedence.clone()
        }
        None => {
            trace!(
                "Could not find precedence for TokenType::{:?} so returning LOWEST.",
                token_type
            );
            ExpressionPrecedence::LOWEST
        }
    }
}
