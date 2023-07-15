use crate::lexical_analysis::model::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Block {
    pub(crate) nodes: Vec<SyntaxTreeNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum SyntaxTreeNode {
    Statement { statement: Statement },
    Expression { expression: Expression },
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Statement {
    Let {
        identifier: String,
        expression: Box<Expression>,
    },
    Return {
        expression: Box<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Expression {
    Identifier {
        identifier: String,
    },
    Integer {
        literal: i64,
    },
    String {
        literal: String,
    },
    NotPrefix {
        right_hand: Box<Expression>,
    },
    MinusPrefix {
        right_hand: Box<Expression>,
    },
    PlusInfix {
        left_hand: Box<Expression>,
        right_hand: Box<Expression>,
    },
    MinusInfix {
        left_hand: Box<Expression>,
        right_hand: Box<Expression>,
    },
    MultiplyInfix {
        left_hand: Box<Expression>,
        right_hand: Box<Expression>,
    },
    Infix {
        left_hand: Box<Expression>,
        operator: Token,
        right_hand: Box<Expression>,
    },
    Boolean {
        literal: bool,
    },
    If {
        condition: Box<Expression>,
        consequence: Box<Block>,
        alternative: Box<Option<Block>>,
    },
    Function {
        parameters: Vec<String>,
        block: Box<Block>,
    },
    Call {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
}
