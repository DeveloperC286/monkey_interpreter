use crate::lexical_analysis::model::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub nodes: Vec<SyntaxTreeNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SyntaxTreeNode {
    Statement { statement: Statement },
    Expression { expression: Expression },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let {
        identifier_token: Token,
        expression: Box<Expression>,
    },
    Return {
        expression: Box<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Identifier {
        identifier_token: Token,
    },
    Integer {
        integer_token: Token,
    },
    Prefix {
        prefix_token: Token,
        right_hand: Box<Expression>,
    },
    Infix {
        left_hand: Box<Expression>,
        operator_token: Token,
        right_hand: Box<Expression>,
    },
    Boolean {
        boolean_token: Token,
    },
    If {
        condition: Box<Expression>,
        consequence: Box<Block>,
        alternative: Box<Option<Block>>,
    },
    Function {
        parameters: Vec<Expression>,
        block: Box<Block>,
    },
    Call {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
}
