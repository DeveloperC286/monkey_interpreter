use crate::lexical_analysis::token::Token;
use crate::syntax_analysis::abstract_syntax_tree::syntax_tree_node::{
    Expression, ExpressionPrecedence,
};
use crate::syntax_analysis::syntax_analysis_context::SyntaxAnalysisContext;

pub fn parse_function_expression(
    mut syntax_analysis_context: SyntaxAnalysisContext,
) -> (SyntaxAnalysisContext, Option<Expression>) {
    debug!("Parsing a function expression.");

    // parse function expression
    assert_token!(syntax_analysis_context, Token::FUNCTION, None);
    let (returned_syntax_analysis_context, parameters) = parse_parameters(syntax_analysis_context);
    syntax_analysis_context = returned_syntax_analysis_context;

    // check function expression was parsed correctly
    let block = match crate::syntax_analysis::expressions::utilities::parse_block(
        syntax_analysis_context,
    ) {
        (returned_syntax_analysis_context, Some(block)) => {
            syntax_analysis_context = returned_syntax_analysis_context;
            block
        }
        (returned_syntax_analysis_context, None) => {
            error!("parse_function_expression could not parse the functions block.");
            return (returned_syntax_analysis_context, None);
        }
    };

    (
        syntax_analysis_context,
        Some(Expression::FUNCTION {
            parameters,
            block: Box::new(block),
        }),
    )
}

fn parse_parameters(
    mut syntax_analysis_context: SyntaxAnalysisContext,
) -> (SyntaxAnalysisContext, Vec<Expression>) {
    debug!("Parsing parameters.");

    assert_token!(
        syntax_analysis_context,
        Token::OPENING_ROUND_BRACKET,
        vec![]
    );
    let mut parameters = vec![];

    if let Some(token) = syntax_analysis_context.tokens.peek() {
        if **token != Token::CLOSING_ROUND_BRACKET {
            loop {
                match super::get_expression(syntax_analysis_context, ExpressionPrecedence::LOWEST) {
                    (returned_syntax_analysis_context, Some(expression)) => {
                        match expression.clone() {
                            Expression::IDENTIFIER {
                                identifier_token: _,
                            } => {
                                parameters.push(expression);
                                syntax_analysis_context = returned_syntax_analysis_context;
                            }
                            _ => {
                                syntax_analysis_context = returned_syntax_analysis_context;
                                syntax_analysis_context.syntax_parsing_errors.push(
                                    "Only allowed Expression::IDENTIFIER in parameters."
                                        .to_string(),
                                );
                            }
                        }
                    }
                    (mut returned_syntax_analysis_context, None) => {
                        returned_syntax_analysis_context
                            .syntax_parsing_errors
                            .push("Unable to parse expression in parameters.".to_string());
                        return (returned_syntax_analysis_context, vec![]);
                    }
                }

                match syntax_analysis_context.tokens.peek() {
                    Some(token) => match token {
                        Token::CLOSING_ROUND_BRACKET => break,
                        Token::COMMA => {
                            syntax_analysis_context.tokens.next();
                        }
                        _ => {
                            syntax_analysis_context.syntax_parsing_errors.push(
                                "Parameters must be comma seperated identifiers.".to_string(),
                            );
                            return (syntax_analysis_context, vec![]);
                        }
                    },
                    None => {
                        return (syntax_analysis_context, vec![]);
                    }
                }
            }
        }
    }

    assert_token!(
        syntax_analysis_context,
        Token::CLOSING_ROUND_BRACKET,
        vec![]
    );
    (syntax_analysis_context, parameters)
}

#[cfg(test)]
mod tests;
