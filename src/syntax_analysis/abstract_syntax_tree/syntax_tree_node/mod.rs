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
}
