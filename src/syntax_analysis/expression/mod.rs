use crate::lexical_analysis::model::token::Token;
use crate::syntax_analysis::model::abstract_syntax_tree::syntax_tree_node::{
    Expression, SyntaxTreeNode,
};
use crate::syntax_analysis::model::expression_precedence::ExpressionPrecedence;
use crate::syntax_analysis::SyntaxAnalysis;

mod function_expression;
mod grouped_expression;
mod if_expression;
mod pratt_parsing;
mod utilities;

impl<'a> SyntaxAnalysis<'a> {
    pub(crate) fn get_expression_node(&mut self) -> Option<SyntaxTreeNode> {
        let expression_option = self.get_expression(ExpressionPrecedence::Lowest);

        semicolon!(self);

        match expression_option {
            Some(expression) => Some(SyntaxTreeNode::Expression { expression }),
            None => None,
        }
    }

    pub(crate) fn get_expression(
        &mut self,
        expression_precedence: ExpressionPrecedence,
    ) -> Option<Expression> {
        debug!("Parsing an expression.");

        let expression: Option<Expression>;

        match self.tokens.peek() {
            Some(token) => match token {
                Token::Identifier { literal: _ } => {
                    debug!("Found an identifier expression.");
                    expression = Some(Expression::Identifier {
                        identifier_token: self.tokens.next().unwrap().clone(),
                    });
                }
                Token::Integer { literal: _ } => {
                    debug!("Found an integer expression.");
                    expression = Some(Expression::Integer {
                        integer_token: self.tokens.next().unwrap().clone(),
                    });
                }
                Token::Not | Token::Minus => {
                    debug!("Found a prefix expression.");
                    let token = self.tokens.next().unwrap().clone();

                    match self.get_expression(ExpressionPrecedence::Prefix) {
                        Some(right_hand) => {
                            expression = Some(Expression::Prefix {
                                prefix_token: token,
                                right_hand: Box::new(right_hand),
                            });
                        }
                        None => {
                            self.syntax_parsing_errors.push(format!(
                                "Syntax error : No right hand expression to prefix {:?}.",
                                token
                            ));
                            return None;
                        }
                    }
                }
                Token::True | Token::False => {
                    debug!("Found an boolean expression.");
                    expression = Some(Expression::Boolean {
                        boolean_token: self.tokens.next().unwrap().clone(),
                    });
                }
                Token::OpeningRoundBracket => {
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
                Token::If => {
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
                Token::Function => {
                    debug!("Found a function expression.");
                    match self.parse_function_expression() {
                        Some(function_expression) => {
                            expression = Some(function_expression);
                        }
                        None => {
                            error!("Error parsing function expression, returning None.");
                            return None;
                        }
                    }
                }
                _ => {
                    self.syntax_parsing_errors.push(format!(
                        "Syntax error : Do not know how to parse {:?} as an expression.",
                        token
                    ));
                    self.tokens.next();
                    return None;
                }
            },
            None => {
                return None;
            }
        }

        self.pratt_parsing(expression, expression_precedence)
    }
}

#[cfg(test)]
mod tests;
