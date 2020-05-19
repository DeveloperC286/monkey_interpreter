use super::super::super::lexical_analysis::token::Token;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Block {
    BLOCK { blocks: Vec<SyntaxTreeNode> },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum SyntaxTreeNode {
    STATEMENT { statement: Statement },
    EXPRESSION { expression: Expression },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Statement {
    LET {
        let_token: Token,
        identifier_token: Token,
    },
    RETURN {
        return_token: Token,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Expression {
    IDENTIFIER {
        identifier_token: Token,
    },
    INTEGER {
        integer_token: Token,
    },
    PREFIX {
        prefix_token: Token,
        right_hand_expression: Box<Expression>,
    },
    INFIX {
        left_hand_expression: Box<Expression>,
        operator_token: Token,
        right_hand_expression: Box<Expression>,
    },
    BOOLEAN {
        boolean_token: Token,
    },
    IF {
        if_token: Token,
        condition: Box<Expression>,
        consequence: Box<Block>,
        alternative: Box<Block>,
    },
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
pub enum ExpressionPrecedence {
    LOWEST,
    EQUALS,
    LESSER_OR_GREATER,
    PLUS,
    MULTIPLY,
    PREFIX,
    FUNCTION_CALL,
}
