use super::lexical_analysis::token::{Token, TokenType};

mod abstract_syntax_tree;
use abstract_syntax_tree::syntax_tree_node::SyntaxTreeNode;
use abstract_syntax_tree::AbstractSyntaxTree;

pub struct SyntaxAnalysis {
    tokenized_code: Vec<Token>,
    current_token_index: usize,
}

impl SyntaxAnalysis {
    pub fn new(tokenized_code: Vec<Token>) -> SyntaxAnalysis {
        return SyntaxAnalysis {
            tokenized_code,
            current_token_index: 0,
        };
    }

    pub fn parse(&mut self) -> AbstractSyntaxTree {
        let mut program: Vec<SyntaxTreeNode> = vec![];

        loop {
            match self.tokenized_code[self.current_token_index].token_type {
                TokenType::EOF => break,
                _ => {
                    let token_option = self.parse_next_node();
                    match token_option {
                        Some(token) => program.push(token),
                        None => {}
                    }
                }
            }

            self.increment_token_index();
        }

        return AbstractSyntaxTree { program };
    }

    fn parse_next_node(&mut self) -> Option<SyntaxTreeNode> {
        match self.tokenized_code[self.current_token_index].token_type {
            TokenType::LET => self.parse_let_statement(),
            TokenType::RETURN => self.parse_return_statement(),
            _ => None,
        }
    }

    fn parse_return_statement(&mut self) -> Option<SyntaxTreeNode> {
        let return_token = self.tokenized_code[self.current_token_index].clone();

        //TODO handle expression.

        self.increment_token_index();
        loop {
            match self.tokenized_code[self.current_token_index].token_type {
                TokenType::SEMI_COLON => break,
                _ => {}
            }
            self.increment_token_index();
        }

        return Some(SyntaxTreeNode::ReturnStatement { return_token });
    }

    fn parse_let_statement(&mut self) -> Option<SyntaxTreeNode> {
        let let_token = self.tokenized_code[self.current_token_index].clone();

        self.increment_token_index();

        if self.tokenized_code[self.current_token_index].token_type != TokenType::IDENTIFIER {
            //TODO add error message about no IDENTIFIER.
            return None;
        }
        let identifier_token = self.tokenized_code[self.current_token_index].clone();

        self.increment_token_index();

        if self.tokenized_code[self.current_token_index].token_type != TokenType::ASSIGN {
            //TODO add error message about no ASSIGN.
            return None;
        }

        //TODO handle expression.

        self.increment_token_index();
        loop {
            match self.tokenized_code[self.current_token_index].token_type {
                TokenType::SEMI_COLON => break,
                _ => {}
            }
            self.increment_token_index();
        }

        return Some(SyntaxTreeNode::LetStatement {
            let_token,
            identifier_token,
        });
    }

    fn increment_token_index(&mut self) {
        self.current_token_index += 1;
    }
}

#[cfg(test)]
mod tests;
