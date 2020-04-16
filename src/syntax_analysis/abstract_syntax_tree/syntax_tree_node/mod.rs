use super::super::super::lexical_analysis::token::Token;

#[derive(Debug, PartialEq)]
pub enum SyntaxTreeNode {
    LetStatement {
        let_token: Token,
        identifier_token: Token,
    },
    ReturnStatement {
        return_token: Token,
    },
    ExpressionStatement {
        expression_token: Token,
        expression: Expression,
    },
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    IDENTIFIER { identifier_token: Token },
    INTEGER { integer_token: Token },
}

#[derive(Debug, Copy, Clone, PartialEq)]
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
