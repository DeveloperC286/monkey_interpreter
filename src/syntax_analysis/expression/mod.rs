use crate::lexical_analysis::model::token::Token;
use crate::syntax_analysis::model::expression_precedence::ExpressionPrecedence;
use crate::syntax_analysis::model::syntax_error::SyntaxError;
use crate::syntax_analysis::model::syntax_tree_node::{Expression, SyntaxTreeNode};
use crate::syntax_analysis::SyntaxAnalysis;

mod function_expression;
mod grouped_expression;
mod if_expression;
mod pratt_parsing;
mod utilities;

impl<'a> SyntaxAnalysis<'a> {
    pub(crate) fn get_expression_node(&mut self) -> Result<SyntaxTreeNode, SyntaxError> {
        let expression = self.get_expression(ExpressionPrecedence::Lowest)?;
        semicolon!(self);
        Ok(SyntaxTreeNode::Expression { expression })
    }

    pub(crate) fn get_expression(
        &mut self,
        expression_precedence: ExpressionPrecedence,
    ) -> Result<Expression, SyntaxError> {
        debug!("Parsing an expression.");

        match self.tokens.peek() {
            Some(token) => match token {
                Token::Identifier { literal } => {
                    debug!("Found a identifier expression.");
                    self.tokens.next();
                    self.pratt_parsing(
                        Expression::Identifier {
                            identifier: literal.clone(),
                        },
                        expression_precedence,
                    )
                }
                Token::String { literal } => {
                    debug!("Found a string expression.");
                    self.tokens.next();
                    self.pratt_parsing(
                        Expression::String {
                            literal: literal.clone(),
                        },
                        expression_precedence,
                    )
                }
                Token::Integer { literal } => {
                    debug!("Found a integer expression.");
                    self.tokens.next();
                    self.pratt_parsing(
                        Expression::Integer { literal: *literal },
                        expression_precedence,
                    )
                }
                Token::Not => {
                    debug!("Found a not prefix expression.");
                    self.tokens.next().unwrap();

                    match self.get_expression(ExpressionPrecedence::Prefix) {
                        Ok(right_hand) => self.pratt_parsing(
                            Expression::NotPrefix {
                                right_hand: Box::new(right_hand),
                            },
                            expression_precedence,
                        ),
                        Err(_) => {
                            // TODO what with other error?
                            Err(SyntaxError::MissingRightHandToPrefixExpression)
                        }
                    }
                }
                Token::Minus => {
                    debug!("Found a minus prefix expression.");
                    self.tokens.next().unwrap();

                    match self.get_expression(ExpressionPrecedence::Prefix) {
                        Ok(right_hand) => self.pratt_parsing(
                            Expression::MinusPrefix {
                                right_hand: Box::new(right_hand),
                            },
                            expression_precedence,
                        ),
                        Err(_) => {
                            // TODO what with other error?
                            Err(SyntaxError::MissingRightHandToPrefixExpression)
                        }
                    }
                }
                Token::True => {
                    debug!("Found a true boolean expression.");
                    self.tokens.next().unwrap();
                    self.pratt_parsing(Expression::Boolean { literal: true }, expression_precedence)
                }
                Token::False => {
                    debug!("Found a false boolean expression.");
                    self.tokens.next().unwrap();
                    self.pratt_parsing(
                        Expression::Boolean { literal: false },
                        expression_precedence,
                    )
                }
                Token::OpeningRoundBracket => {
                    debug!("Found a grouped expression.");
                    let grouped_expression = self.parse_grouped_expression()?;
                    self.pratt_parsing(grouped_expression, expression_precedence)
                }
                Token::If => {
                    debug!("Found a if expression.");
                    let if_expression = self.parse_if_expression()?;
                    self.pratt_parsing(if_expression, expression_precedence)
                }
                Token::Function => {
                    debug!("Found a function expression.");
                    let function_expression = self.parse_function_expression()?;
                    self.pratt_parsing(function_expression, expression_precedence)
                }
                _ => Err(SyntaxError::UnparsableAsExpression((*token).clone())),
            },
            None => Err(SyntaxError::NoTokenToParse),
        }
    }
}
