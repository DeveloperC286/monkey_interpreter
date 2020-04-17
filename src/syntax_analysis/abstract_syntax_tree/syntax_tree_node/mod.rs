use super::super::super::lexical_analysis::token::Token;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum SyntaxTreeNode {
    LET_STATEMENT {
        let_token: Token,
        identifier_token: Token,
    },
    RETURN_STATEMENT {
        return_token: Token,
    },
    EXPRESSION_STATEMENT {
        initial_expression_token: Token,
        expression: Expression,
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
