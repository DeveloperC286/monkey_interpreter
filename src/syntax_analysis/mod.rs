use super::lexical_analysis::token::Token;

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
        let mut program = vec![];

        return AbstractSyntaxTree { program };
    }
}

#[cfg(test)]
mod tests;
