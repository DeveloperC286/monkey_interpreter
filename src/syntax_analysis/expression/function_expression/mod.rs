use crate::lexical_analysis::model::token::Token;
use crate::syntax_analysis::model::abstract_syntax_tree::syntax_tree_node::Expression;
use crate::syntax_analysis::model::expression_precedence::ExpressionPrecedence;
use crate::syntax_analysis::SyntaxAnalysis;

impl<'a> SyntaxAnalysis<'a> {
    pub(crate) fn parse_function_expression(&mut self) -> Option<Expression> {
        debug!("Parsing a function expression.");

        // parse function expression
        assert_token!(self, Token::Function, None);
        let parameters = self.parse_parameters();

        // check function expression was parsed correctly
        let block = match self.parse_block() {
            Some(block) => block,
            None => {
                error!("parse_function_expression could not parse the functions block.");
                return None;
            }
        };

        Some(Expression::Function {
            parameters,
            block: Box::new(block),
        })
    }

    fn parse_parameters(&mut self) -> Vec<String> {
        debug!("Parsing parameters.");

        assert_token!(self, Token::OpeningRoundBracket, vec![]);
        let mut parameters = vec![];

        if let Some(token) = self.tokens.peek() {
            if **token != Token::ClosingRoundBracket {
                loop {
                    match self.get_expression(ExpressionPrecedence::Lowest) {
                        Some(expression) => match expression {
                            Expression::Identifier { identifier } => {
                                parameters.push(identifier);
                            }
                            _ => {
                                self.syntax_parsing_errors.push(
                                    "Only allowed Expression::IDENTIFIER in parameters."
                                        .to_string(),
                                );
                            }
                        },
                        None => {
                            self.syntax_parsing_errors
                                .push("Unable to parse expression in parameters.".to_string());
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
                                self.syntax_parsing_errors.push(
                                    "Parameters must be comma seperated identifiers.".to_string(),
                                );
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
        parameters
    }
}

#[cfg(test)]
mod tests;
