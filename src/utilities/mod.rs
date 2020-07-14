use crate::lexical_analysis::token::TokenType;
use crate::syntax_analysis::abstract_syntax_tree::syntax_tree_node::ExpressionPrecedence;
use std::collections::HashMap;

lazy_static! {
    pub static ref KEYWORDS: HashMap<String, TokenType> = {
        let mut m = HashMap::new();
        m.insert("fn".to_string(), TokenType::FUNCTION);
        m.insert("let".to_string(), TokenType::LET);
        m.insert("true".to_string(), TokenType::TRUE);
        m.insert("false".to_string(), TokenType::FALSE);
        m.insert("if".to_string(), TokenType::IF);
        m.insert("else".to_string(), TokenType::ELSE);
        m.insert("return".to_string(), TokenType::RETURN);
        m
    };
}

lazy_static! {
    pub static ref TOKEN_PRECEDENCES: HashMap<TokenType, ExpressionPrecedence> = {
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
