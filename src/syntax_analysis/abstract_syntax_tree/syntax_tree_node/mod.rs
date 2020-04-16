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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Expression {
    IDENTIFIER {
        expression_precedence: ExpressionPrecedence,
        identifier_token: Token,
    },
    INTEGER {
        expression_precedence: ExpressionPrecedence,
        integer_token: Token,
    },
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
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
