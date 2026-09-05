use anyhow::Context;
use log::{debug, trace};

use crate::lexical_analysis::Token;
use crate::syntax_analysis::SyntaxAnalysis;
use crate::syntax_analysis::expression_precedence::ExpressionPrecedence;
use crate::syntax_analysis::{Expression, SyntaxTreeNode};

mod function_expression;
mod grouped_expression;
mod if_expression;
mod pratt_parsing;
mod utilities;

impl SyntaxAnalysis<'_> {
    pub(super) fn get_expression_node(&mut self) -> anyhow::Result<SyntaxTreeNode> {
        let expression = self.get_expression(ExpressionPrecedence::Lowest)?;
        semicolon!(self);
        Ok(SyntaxTreeNode::Expression { expression })
    }

    pub(super) fn get_expression(
        &mut self,
        expression_precedence: ExpressionPrecedence,
    ) -> anyhow::Result<Expression> {
        debug!("Parsing an expression.");

        match self.tokens.peek() {
            None => anyhow::bail!("No token to parse."),
            Some(Token::OpeningRoundBracket) => {
                debug!("Found a grouped expression.");
                let grouped_expression = self.parse_grouped_expression()?;
                self.pratt_parsing(grouped_expression, expression_precedence)
            }
            Some(Token::If) => {
                debug!("Found a if expression.");
                let if_expression = self.parse_if_expression()?;
                self.pratt_parsing(if_expression, expression_precedence)
            }
            Some(Token::Function) => {
                debug!("Found a function expression.");
                let function_expression = self.parse_function_expression()?;
                self.pratt_parsing(function_expression, expression_precedence)
            }
            Some(token) => {
                // Every remaining expression starts by consuming the token it was
                // recognised by, the delegating arms above consume their own.
                let token = *token;
                self.tokens.next();

                match token {
                    Token::Identifier { literal } => {
                        debug!("Found a identifier expression.");
                        self.pratt_parsing(
                            Expression::Identifier {
                                identifier: literal.clone(),
                            },
                            expression_precedence,
                        )
                    }
                    Token::String { literal } => {
                        debug!("Found a string expression.");
                        self.pratt_parsing(
                            Expression::String {
                                literal: literal.clone(),
                            },
                            expression_precedence,
                        )
                    }
                    Token::Integer { literal } => {
                        debug!("Found a integer expression.");
                        self.pratt_parsing(
                            Expression::Integer { literal: *literal },
                            expression_precedence,
                        )
                    }
                    Token::Not => {
                        debug!("Found a not prefix expression.");

                        let right_hand = self
                            .get_expression(ExpressionPrecedence::Prefix)
                            .context("A prefix expression must have a right hand expression.")?;
                        self.pratt_parsing(
                            Expression::NotPrefix {
                                right_hand: Box::new(right_hand),
                            },
                            expression_precedence,
                        )
                    }
                    Token::Minus => {
                        debug!("Found a minus prefix expression.");

                        let right_hand = self
                            .get_expression(ExpressionPrecedence::Prefix)
                            .context("A prefix expression must have a right hand expression.")?;
                        self.pratt_parsing(
                            Expression::MinusPrefix {
                                right_hand: Box::new(right_hand),
                            },
                            expression_precedence,
                        )
                    }
                    Token::True => {
                        debug!("Found a true boolean expression.");
                        self.pratt_parsing(
                            Expression::Boolean { literal: true },
                            expression_precedence,
                        )
                    }
                    Token::False => {
                        debug!("Found a false boolean expression.");
                        self.pratt_parsing(
                            Expression::Boolean { literal: false },
                            expression_precedence,
                        )
                    }
                    _ => anyhow::bail!("Do not know how to parse {:?} as an expression.", token),
                }
            }
        }
    }
}
