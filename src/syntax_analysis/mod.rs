use super::lexical_analysis::token::{Token, TokenType};

mod abstract_syntax_tree;
use abstract_syntax_tree::AbstractSyntaxTree;

pub struct SyntaxAnalysis {
    tokenized_code: Vec<Token>,
    current_token_index: i32,
}

impl SyntaxAnalysis {
    pub fn new(tokenized_code: Vec<Token>) -> SyntaxAnalysis {
        return SyntaxAnalysis {
            tokenized_code,
            current_token_index: -1,
        };
    }

    pub fn parse(&self) -> AbstractSyntaxTree {
        return AbstractSyntaxTree { program: vec![] };
    }
}

#[cfg(test)]
mod tests;
