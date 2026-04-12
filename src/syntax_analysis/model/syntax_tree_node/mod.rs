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
        identifier: String,
        expression: Box<Expression>,
    },
    Return {
        expression: Box<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum InfixOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Equals,
    NotEquals,
    LesserThan,
    GreaterThan,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
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
    Infix {
        left_hand: Box<Expression>,
        operator: InfixOperator,
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
