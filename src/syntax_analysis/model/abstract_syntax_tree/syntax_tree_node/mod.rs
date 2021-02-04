use crate::lexical_analysis::model::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub nodes: Vec<SyntaxTreeNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SyntaxTreeNode {
    STATEMENT { statement: Statement },
    EXPRESSION { expression: Expression },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    LET {
        identifier_token: Token,
        expression: Box<Expression>,
    },
    RETURN {
        expression: Box<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq)]
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