use crate::lexical_analysis::model::token::Token;
use crate::syntax_analysis::model::abstract_syntax_tree::syntax_tree_node::{
    Expression, SyntaxTreeNode,
};
use crate::syntax_analysis::model::expression_precedence::ExpressionPrecedence;
use crate::syntax_analysis::model::syntax_analysis_context::SyntaxAnalysisContext;

mod function_expression;
mod grouped_expression;
mod if_expression;
mod pratt_parsing;
mod utilities;

pub fn get_expression_node(
    mut syntax_analysis_context: SyntaxAnalysisContext,
) -> (SyntaxAnalysisContext, Option<SyntaxTreeNode>) {
    let (returned_syntax_analysis_context, expression_option) =
        get_expression(syntax_analysis_context, ExpressionPrecedence::LOWEST);
    syntax_analysis_context = returned_syntax_analysis_context;

    semicolon!(syntax_analysis_context);

    match expression_option {
        Some(expression) => (
            syntax_analysis_context,
            Some(SyntaxTreeNode::EXPRESSION { expression }),
        ),
        None => (syntax_analysis_context, None),
    }
}

pub fn get_expression(
    mut syntax_analysis_context: SyntaxAnalysisContext,
    expression_precedence: ExpressionPrecedence,
) -> (SyntaxAnalysisContext, Option<Expression>) {
    debug!("Parsing an expression.");

    let expression: Option<Expression>;

    match syntax_analysis_context.tokens.peek() {
        Some(token) => match token {
            Token::Identifier { literal: _ } => {
                debug!("Found an identifier expression.");
                expression = Some(Expression::IDENTIFIER {
                    identifier_token: syntax_analysis_context.tokens.next().unwrap().clone(),
                });
            }
            Token::Integer { literal: _ } => {
                debug!("Found an integer expression.");
                expression = Some(Expression::INTEGER {
                    integer_token: syntax_analysis_context.tokens.next().unwrap().clone(),
                });
            }
            Token::Not | Token::Minus => {
                debug!("Found a prefix expression.");
                let token = syntax_analysis_context.tokens.next().unwrap().clone();

                match get_expression(syntax_analysis_context, ExpressionPrecedence::PREFIX) {
                    (returned_syntax_analysis_context, Some(right_hand)) => {
                        syntax_analysis_context = returned_syntax_analysis_context;
                        expression = Some(Expression::PREFIX {
                            prefix_token: token,
                            right_hand: Box::new(right_hand),
                        });
                    }
                    (mut returned_syntax_analysis_context, None) => {
                        returned_syntax_analysis_context
                            .syntax_parsing_errors
                            .push(format!(
                                "Syntax error : No right hand expression to prefix {:?}.",
                                token
                            ));
                        return (returned_syntax_analysis_context, None);
                    }
                }
            }
            Token::True | Token::False => {
                debug!("Found an boolean expression.");
                expression = Some(Expression::BOOLEAN {
                    boolean_token: syntax_analysis_context.tokens.next().unwrap().clone(),
                });
            }
            Token::OpeningRoundBracket => {
                debug!("Found a grouped expression.");
                match grouped_expression::parse_grouped_expression(syntax_analysis_context) {
                    (returned_syntax_analysis_context, Some(grouped_expression)) => {
                        syntax_analysis_context = returned_syntax_analysis_context;
                        expression = Some(grouped_expression);
                    }
                    (returned_syntax_analysis_context, None) => {
                        error!("Error parsing grouped expression, returning None.");
                        return (returned_syntax_analysis_context, None);
                    }
                }
            }
            Token::If => {
                debug!("Found an if expression.");
                match if_expression::parse_if_expression(syntax_analysis_context) {
                    (returned_syntax_analysis_context, Some(if_expression)) => {
                        syntax_analysis_context = returned_syntax_analysis_context;
                        expression = Some(if_expression);
                    }
                    (returned_syntax_analysis_context, None) => {
                        error!("Error parsing if expression, returning None.");
                        return (returned_syntax_analysis_context, None);
                    }
                }
            }
            Token::Function => {
                debug!("Found a function expression.");
                match function_expression::parse_function_expression(syntax_analysis_context) {
                    (returned_syntax_analysis_context, Some(function_expression)) => {
                        syntax_analysis_context = returned_syntax_analysis_context;
                        expression = Some(function_expression);
                    }
                    (returned_syntax_analysis_context, None) => {
                        error!("Error parsing function expression, returning None.");
                        return (returned_syntax_analysis_context, None);
                    }
                }
            }
            _ => {
                syntax_analysis_context.syntax_parsing_errors.push(format!(
                    "Syntax error : Do not know how to parse {:?} as an expression.",
                    token
                ));
                syntax_analysis_context.tokens.next();
                return (syntax_analysis_context, None);
            }
        },
        None => {
            return (syntax_analysis_context, None);
        }
    }

    pratt_parsing::pratt_parsing(syntax_analysis_context, expression, expression_precedence)
}

#[cfg(test)]
mod tests;
