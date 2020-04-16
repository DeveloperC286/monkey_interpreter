use super::lexical_analysis::token::{Token, TokenType};

pub mod abstract_syntax_tree;
use abstract_syntax_tree::syntax_tree_node::{Expression, ExpressionPrecedence, SyntaxTreeNode};
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
            debug!(
                "Syntax parsing the Token '{:?}' at index {}.",
                self.tokenized_code[self.current_token_index], self.current_token_index
            );
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
        }

        return AbstractSyntaxTree { program };
    }

    fn parse_next_node(&mut self) -> Option<SyntaxTreeNode> {
        match self.tokenized_code[self.current_token_index].token_type {
            TokenType::LET => self.parse_let_statement(),
            TokenType::RETURN => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_expression_statement(&mut self) -> Option<SyntaxTreeNode> {
        debug!("Parsing an expression statement.");
        let initial_expression_token = self.tokenized_code[self.current_token_index].clone();

        let expression_option = self.parse_expression(ExpressionPrecedence::LOWEST);

        if self.tokenized_code[self.current_token_index].token_type == TokenType::SEMI_COLON {
            trace!("Jumping over semi colon.");
            self.increment_token_index();
        }

        match expression_option {
            Some(expression) => {
                return Some(SyntaxTreeNode::EXPRESSION_STATEMENT {
                    initial_expression_token,
                    expression,
                });
            }
            None => return None,
        }
    }

    fn parse_expression(
        &mut self,
        expression_precedence: ExpressionPrecedence,
    ) -> Option<Expression> {
        debug!("Parsing an expression.");
        let token = self.tokenized_code[self.current_token_index].clone();
        self.increment_token_index();

        match token.token_type {
            TokenType::IDENTIFIER => {
                debug!("Returning an identifier expression.");
                return Some(Expression::IDENTIFIER {
                    expression_precedence,
                    identifier_token: token,
                });
            }
            TokenType::INTEGER => {
                debug!("Returning an integer expression.");
                return Some(Expression::INTEGER {
                    expression_precedence,
                    integer_token: token,
                });
            }
            _ => {
                debug!("Unable to parse expression.");
                return None;
            }
        }
    }

    fn parse_return_statement(&mut self) -> Option<SyntaxTreeNode> {
        debug!("Parsing a return statement.");
        let return_token = self.tokenized_code[self.current_token_index].clone();

        //TODO handle expression.

        self.increment_token_index();

        loop {
            match self.tokenized_code[self.current_token_index].token_type {
                TokenType::SEMI_COLON => {
                    self.increment_token_index();
                    break;
                }
                _ => {
                    self.increment_token_index();
                }
            }
        }

        return Some(SyntaxTreeNode::RETURN_STATEMENT { return_token });
    }

    fn parse_let_statement(&mut self) -> Option<SyntaxTreeNode> {
        debug!("Parsing a let statement.");
        let let_token = self.tokenized_code[self.current_token_index].clone();

        self.increment_token_index();

        if self.tokenized_code[self.current_token_index].token_type != TokenType::IDENTIFIER {
            error!(
                "No TokenType::IDENTIFIER at '{}' for the let statement.",
                self.current_token_index
            );
            //TODO add error message about no IDENTIFIER.
            return None;
        }
        let identifier_token = self.tokenized_code[self.current_token_index].clone();

        self.increment_token_index();

        if self.tokenized_code[self.current_token_index].token_type != TokenType::ASSIGN {
            error!(
                "No TokenType::ASSIGN at '{}' for the let statement.",
                self.current_token_index
            );
            //TODO add error message about no ASSIGN.
            return None;
        }

        //TODO handle expression.

        self.increment_token_index();

        loop {
            match self.tokenized_code[self.current_token_index].token_type {
                TokenType::SEMI_COLON => {
                    self.increment_token_index();
                    break;
                }
                _ => {
                    self.increment_token_index();
                }
            }
        }

        return Some(SyntaxTreeNode::LET_STATEMENT {
            let_token,
            identifier_token,
        });
    }

    fn increment_token_index(&mut self) {
        trace!("Incrementing the token index.");
        self.current_token_index += 1;
    }
}

#[cfg(test)]
mod tests;
