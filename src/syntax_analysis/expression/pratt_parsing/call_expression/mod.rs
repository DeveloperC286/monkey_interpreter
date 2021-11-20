use crate::lexical_analysis::model::token::Token;
use crate::syntax_analysis::model::abstract_syntax_tree::syntax_tree_node::Expression;
use crate::syntax_analysis::model::expression_precedence::ExpressionPrecedence;
use crate::syntax_analysis::SyntaxAnalysis;

impl<'a> SyntaxAnalysis<'a> {
    pub(crate) fn parse_call_expression(&mut self, function: Expression) -> Option<Expression> {
        debug!("Parsing a call expression.");

        // parse call expression
        let arguments = self.parse_arguments();

        // check call expression was correctly called
        match &function {
            Expression::Identifier { identifier: _ } => {}
            _ => {
                error!("parse_call_expression called with the function not being an Expression::IDENTIFIER.");
                return None;
            }
        }

        Some(Expression::Call {
            function: Box::new(function),
            arguments,
        })
    }

    fn parse_arguments(&mut self) -> Vec<Expression> {
        debug!("Parsing arguments.");

        assert_token!(self, Token::OpeningRoundBracket, vec![]);
        let mut arguments = vec![];

        if let Some(token) = self.tokens.peek() {
            if **token != Token::ClosingRoundBracket {
                loop {
                    match self.get_expression(ExpressionPrecedence::Lowest) {
                        Some(expression) => {
                            arguments.push(expression);
                        }
                        None => {
                            self.syntax_parsing_errors
                                .push("Unable to parse expression in arguments.".to_string());
                            return vec![];
                        }
                    }

                    match self.tokens.peek() {
                        Some(token) => match token {
                            Token::ClosingRoundBracket => break,
                            Token::Comma => {
                                self.tokens.next();
                            }
                            _ => {
                                return vec![];
                            }
                        },
                        None => {
                            return vec![];
                        }
                    }
                }
            }
        }

        assert_token!(self, Token::ClosingRoundBracket, vec![]);
        arguments
    }
}

#[cfg(test)]
mod tests;
