use serde::{Deserialize, Serialize};

use crate::lexical_analysis::token::Token;

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
        identifier_token: Token,
        expression: Box<Expression>,
    },
    RETURN {
        expression: Box<Expression>,
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
        right_hand: Box<Expression>,
    },
    INFIX {
        left_hand: Box<Expression>,
        operator_token: Token,
        right_hand: Box<Expression>,
    },
    BOOLEAN {
        boolean_token: Token,
    },
    IF {
        condition: Box<Expression>,
        consequence: Box<Block>,
        alternative: Box<Option<Block>>,
    },
    FUNCTION {
        parameters: Vec<Expression>,
        block: Box<Block>,
    },
    CALL {
        function: Box<Expression>,
        arguments: Vec<Expression>,
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
    CALL,
}
