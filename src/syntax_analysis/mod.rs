use super::lexical_analysis::token::{Token, TokenType};

pub mod abstract_syntax_tree;
use abstract_syntax_tree::syntax_tree_node::{Expression, ExpressionPrecedence, SyntaxTreeNode};
use abstract_syntax_tree::AbstractSyntaxTree;

use std::collections::HashMap;

lazy_static! {
    static ref TOKEN_PRECEDENCES: HashMap<TokenType, ExpressionPrecedence> = {
        let mut m = HashMap::new();
        m.insert(TokenType::EQUALS, ExpressionPrecedence::EQUALS);
        m.insert(TokenType::NOT_EQUALS, ExpressionPrecedence::EQUALS);
        m.insert(
            TokenType::LESSER_THAN,
            ExpressionPrecedence::LESSER_OR_GREATER,
        );
        m.insert(
            TokenType::GREATER_THAN,
            ExpressionPrecedence::LESSER_OR_GREATER,
        );
        m.insert(TokenType::PLUS, ExpressionPrecedence::PLUS);
        m.insert(TokenType::MINUS, ExpressionPrecedence::PLUS);
        m.insert(TokenType::MULTIPLY, ExpressionPrecedence::MULTIPLY);
        m.insert(TokenType::DIVIDE, ExpressionPrecedence::MULTIPLY);
        m
    };
}

#[macro_use]
mod macros;

pub struct SyntaxAnalysis {
    tokens: Vec<Token>,
    syntax_parsing_errors: Vec<String>,
    current_token_index: usize,
    current_token: Token,
}

impl SyntaxAnalysis {
    fn new(tokens: Vec<Token>) -> SyntaxAnalysis {
        let mut current_token = Token {
            token_type: TokenType::EOF,
            literal: "".to_string(),
        };

        if tokens.len() > 0 {
            current_token = tokens.get(0).unwrap().clone();
        }

        return SyntaxAnalysis {
            syntax_parsing_errors: vec![],
            current_token_index: 0,
            current_token,
            tokens,
        };
    }

    pub fn get_abstract_syntax_tree(tokens: Vec<Token>) -> AbstractSyntaxTree {
        let mut syntax_analysis = SyntaxAnalysis::new(tokens);
        return syntax_analysis.parse();
    }

    fn parse(&mut self) -> AbstractSyntaxTree {
        let mut program: Vec<SyntaxTreeNode> = vec![];

        loop {
            debug!(
                "Syntax parsing the Token '{:?}' at index {}.",
                self.current_token, self.current_token_index
            );
            match self.current_token.token_type {
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

        return AbstractSyntaxTree {
            program,
            syntax_parsing_errors: self.syntax_parsing_errors.clone(),
        };
    }

    fn parse_next_node(&mut self) -> Option<SyntaxTreeNode> {
        match self.current_token.token_type {
            TokenType::LET => self.parse_let_statement(),
            TokenType::RETURN => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_expression_statement(&mut self) -> Option<SyntaxTreeNode> {
        debug!("Parsing an expression statement.");

        let initial_expression_token = self.current_token.clone();

        let expression_option = self.parse_expression(ExpressionPrecedence::LOWEST);

        if self.current_token.token_type == TokenType::SEMI_COLON {
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

        let token = self.current_token.clone();
        self.increment_token_index();

        let mut expression: Option<Expression> = None;

        match token.token_type {
            TokenType::IDENTIFIER => {
                debug!("Returning an identifier expression.");
                expression = Some(Expression::IDENTIFIER {
                    identifier_token: token,
                });
            }
            TokenType::INTEGER => {
                debug!("Returning an integer expression.");
                expression = Some(Expression::INTEGER {
                    integer_token: token,
                });
            }
            TokenType::NOT | TokenType::MINUS => {
                debug!("Returning a prefix expression.");
                match self.parse_expression(ExpressionPrecedence::PREFIX) {
                    Some(right_hand_expression) => {
                        expression = Some(Expression::PREFIX {
                            prefix_token: token,
                            right_hand_expression: Box::new(right_hand_expression),
                        });
                    }
                    None => {
                        self.syntax_parsing_errors.push(format!(
                            "Syntax error : No right hand expression to prefix {:?}.",
                            token.token_type
                        ));
                        return expression;
                    }
                }
            }
            TokenType::TRUE | TokenType::FALSE => {
                debug!("Returning an boolean expression.");
                expression = Some(Expression::BOOLEAN {
                    boolean_token: token,
                });
            }
            TokenType::OPENING_ROUND_BRACKET => {
                debug!("Returning a grouped expression.");
                match self.parse_grouped_expression() {
                    Some(grouped_expression) => {
                        expression = Some(grouped_expression);
                    }
                    None => {
                        return expression;
                    }
                }
            }
            _ => {
                self.syntax_parsing_errors.push(format!(
                    "Syntax error : Do not know how to parse {:?} as an expression.",
                    token.token_type
                ));
                return expression;
            }
        }

        //Pratt Parsing.
        loop {
            if self.current_token.token_type == TokenType::SEMI_COLON {
                break;
            }

            if !(expression_precedence < self.get_current_expression_precedence()) {
                break;
            }

            match self.current_token.token_type {
                TokenType::PLUS
                | TokenType::MINUS
                | TokenType::DIVIDE
                | TokenType::MULTIPLY
                | TokenType::EQUALS
                | TokenType::NOT_EQUALS
                | TokenType::LESSER_THAN
                | TokenType::GREATER_THAN => {
                    expression = self.parse_inflix_expression(expression.clone().unwrap());
                }
                _ => {
                    break;
                }
            }
        }

        return expression;
    }

    fn parse_grouped_expression(&mut self) -> Option<Expression> {
        debug!("Parsing a grouped expression.");

        // As only called from parse_expression which checks expect_token!(self, TokenType::OPENING_ROUND_BRACKET);
        let grouped_expression = self.parse_expression(ExpressionPrecedence::LOWEST);
        expect_token!(self, TokenType::CLOSING_ROUND_BRACKET);

        return grouped_expression;
    }

    fn parse_inflix_expression(&mut self, left_hand_expression: Expression) -> Option<Expression> {
        debug!("Parsing a inflix expression.");

        let operator_token = self.current_token.clone();
        let precedence = self.get_current_expression_precedence();
        self.increment_token_index();

        match self.parse_expression(precedence) {
            Some(right_hand_expression) => {
                return Some(Expression::INFIX {
                    left_hand_expression: Box::new(left_hand_expression),
                    operator_token,
                    right_hand_expression: Box::new(right_hand_expression),
                });
            }
            None => return None,
        }
    }

    fn get_current_expression_precedence(&mut self) -> ExpressionPrecedence {
        match TOKEN_PRECEDENCES.get(&self.current_token.token_type) {
            Some(expression_precedence) => {
                trace!("get_current_expression_precedence() found the precedence and is returning {:?}.", expression_precedence);
                return expression_precedence.clone();
            }
            None => {
                trace!("get_current_expression_precedence() couldn't find precedence so returning LOWEST.");
                return ExpressionPrecedence::LOWEST;
            }
        }
    }

    fn parse_return_statement(&mut self) -> Option<SyntaxTreeNode> {
        debug!("Parsing a return statement.");

        let return_token = self.current_token.clone();
        expect_token!(self, TokenType::RETURN);

        //TODO handle expression.
        loop {
            match self.current_token.token_type {
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

        let let_token = self.current_token.clone();
        expect_token!(self, TokenType::LET);

        let identifier_token = self.current_token.clone();
        expect_token!(self, TokenType::IDENTIFIER);

        expect_token!(self, TokenType::ASSIGN);

        //TODO handle expression.
        loop {
            match self.current_token.token_type {
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
        self.current_token_index += 1;
        trace!(
            "Incremented the current token index to '{}'.",
            self.current_token_index
        );

        if self.current_token_index < self.tokens.len() {
            self.current_token = self.tokens[self.current_token_index].clone();
        } else {
            error!("self.current_token_index >= self.tokens.len(), so setting current_token to EOF Token.");
            self.current_token = Token {
                token_type: TokenType::EOF,
                literal: "".to_string(),
            };
        }
    }
}

#[cfg(test)]
mod tests;
