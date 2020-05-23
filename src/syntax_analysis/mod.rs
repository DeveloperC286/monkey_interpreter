use super::lexical_analysis::token::{Token, TokenType};

pub mod abstract_syntax_tree;
use abstract_syntax_tree::syntax_tree_node::{
    Block, Expression, ExpressionPrecedence, Statement, SyntaxTreeNode,
};
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
        let mut abstract_syntax_tree: Vec<SyntaxTreeNode> = vec![];

        loop {
            debug!(
                "Syntax parsing the Token '{:?}' at index {}.",
                self.current_token, self.current_token_index
            );
            match self.current_token.token_type {
                TokenType::EOF => break,
                _ => match self.parse_next_node() {
                    Some(token) => abstract_syntax_tree.push(token),
                    None => {}
                },
            }
        }

        return AbstractSyntaxTree {
            abstract_syntax_tree,
            syntax_parsing_errors: self.syntax_parsing_errors.clone(),
        };
    }

    fn parse_next_node(&mut self) -> Option<SyntaxTreeNode> {
        debug!("Parsing next SyntaxTreeNode.");
        match self.current_token.token_type {
            TokenType::LET => self.parse_let_statement(),
            TokenType::RETURN => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_expression_statement(&mut self) -> Option<SyntaxTreeNode> {
        let expression_option = self.parse_expression(ExpressionPrecedence::LOWEST);

        if self.current_token.token_type == TokenType::SEMI_COLON {
            trace!("Ignoring expression's semi colon.");
            self.increment_token_index();
        }

        match expression_option {
            Some(expression) => {
                return Some(SyntaxTreeNode::EXPRESSION { expression });
            }
            None => return None,
        }
    }

    fn parse_expression(
        &mut self,
        expression_precedence: ExpressionPrecedence,
    ) -> Option<Expression> {
        debug!("Parsing an expression.");

        let mut expression: Option<Expression> = None;

        match self.current_token.token_type {
            TokenType::IDENTIFIER => {
                debug!("Found an identifier expression.");
                expression = Some(Expression::IDENTIFIER {
                    identifier_token: self.current_token.clone(),
                });
                self.increment_token_index();
            }
            TokenType::INTEGER => {
                debug!("Found an integer expression.");
                expression = Some(Expression::INTEGER {
                    integer_token: self.current_token.clone(),
                });
                self.increment_token_index();
            }
            TokenType::NOT | TokenType::MINUS => {
                debug!("Found a prefix expression.");
                let token = self.current_token.clone();
                self.increment_token_index();

                match self.parse_expression(ExpressionPrecedence::PREFIX) {
                    Some(right_hand_expression) => {
                        expression = Some(Expression::PREFIX {
                            prefix_token: token,
                            right_hand_expression: Box::new(right_hand_expression),
                        });
                    }
                    None => {
                        syntax_error!(
                            self,
                            format!(
                                "Syntax error : No right hand expression to prefix {:?}.",
                                token.token_type
                            )
                        );

                        return None;
                    }
                }
            }
            TokenType::TRUE | TokenType::FALSE => {
                debug!("Found an boolean expression.");
                expression = Some(Expression::BOOLEAN {
                    boolean_token: self.current_token.clone(),
                });
                self.increment_token_index();
            }
            TokenType::OPENING_ROUND_BRACKET => {
                debug!("Found a grouped expression.");
                match self.parse_grouped_expression() {
                    Some(grouped_expression) => {
                        expression = Some(grouped_expression);
                    }
                    None => {
                        error!("Error parsing grouped expression, returning None.");
                        return None;
                    }
                }
            }
            TokenType::IF => {
                debug!("Found an if expression.");
                match self.parse_if_expression() {
                    Some(if_expression) => {
                        expression = Some(if_expression);
                    }
                    None => {
                        error!("Error parsing if expression, returning None.");
                        return None;
                    }
                }
            }
            _ => {
                syntax_error!(
                    self,
                    format!(
                        "Syntax error : Do not know how to parse {:?} as an expression.",
                        self.current_token.token_type
                    )
                );
                self.increment_token_index();
                return None;
            }
        }

        return self.pratt_parsing(expression, expression_precedence);
    }

    fn pratt_parsing(
        &mut self,
        mut expression: Option<Expression>,
        expression_precedence: ExpressionPrecedence,
    ) -> Option<Expression> {
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

    fn parse_if_expression(&mut self) -> Option<Expression> {
        debug!("Parsing a if expression.");

        // parse if expression
        expect_token!(self, TokenType::IF);
        expect_token!(self, TokenType::OPENING_ROUND_BRACKET);
        let condition_option = self.parse_expression(ExpressionPrecedence::LOWEST);
        expect_token!(self, TokenType::CLOSING_ROUND_BRACKET);
        let consequence_option = self.parse_block();
        let mut alternative = None;

        if self.current_token.token_type == TokenType::ELSE {
            expect_token!(self, TokenType::ELSE);
            alternative = self.parse_block();
        }

        // check if expression was parsed correctly
        let condition = match condition_option {
            Some(condition) => condition,
            None => {
                return None;
            }
        };

        let consequence = match consequence_option {
            Some(consequence) => consequence,
            None => {
                return None;
            }
        };

        return Some(Expression::IF {
            condition: Box::new(condition),
            consequence: Box::new(consequence),
            alternative: Box::new(alternative),
        });
    }

    fn parse_block(&mut self) -> Option<Block> {
        debug!("Parsing a block.");
        expect_token!(self, TokenType::OPENING_CURLY_BRACKET);
        let mut blocks = vec![];

        loop {
            match self.current_token.token_type {
                TokenType::CLOSING_CURLY_BRACKET | TokenType::EOF => break,
                _ => match self.parse_next_node() {
                    Some(token) => blocks.push(token),
                    None => {}
                },
            }
        }

        expect_token!(self, TokenType::CLOSING_CURLY_BRACKET);

        return Some(Block::BLOCK { blocks });
    }

    fn parse_grouped_expression(&mut self) -> Option<Expression> {
        debug!("Parsing a grouped expression.");

        expect_token!(self, TokenType::OPENING_ROUND_BRACKET);
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

        return Some(SyntaxTreeNode::STATEMENT {
            statement: Statement::RETURN { return_token },
        });
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

        return Some(SyntaxTreeNode::STATEMENT {
            statement: Statement::LET {
                let_token,
                identifier_token,
            },
        });
    }

    fn increment_token_index(&mut self) {
        self.current_token_index += 1;

        if self.current_token_index < self.tokens.len() {
            self.current_token = self.tokens[self.current_token_index].clone();
            trace!(
                "self.current_token_index [{}], self.current_token [{:?}]",
                self.current_token_index,
                self.current_token
            );
        } else {
            error!("self.current_token_index [{}] >= self.tokens.len(), so setting current_token to EOF Token.", self.current_token_index);
            self.current_token = Token {
                token_type: TokenType::EOF,
                literal: "".to_string(),
            };
        }
    }
}

#[cfg(test)]
mod tests;
