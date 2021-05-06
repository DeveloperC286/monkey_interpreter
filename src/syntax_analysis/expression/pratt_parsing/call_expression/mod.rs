use crate::lexical_analysis::model::token::Token;
use crate::syntax_analysis::model::abstract_syntax_tree::syntax_tree_node::Expression;
use crate::syntax_analysis::model::expression_precedence::ExpressionPrecedence;
use crate::syntax_analysis::model::syntax_analysis_context::SyntaxAnalysisContext;

pub fn parse_call_expression(
    mut syntax_analysis_context: SyntaxAnalysisContext,
    function: Expression,
) -> (SyntaxAnalysisContext, Option<Expression>) {
    debug!("Parsing a call expression.");

    // parse call expression
    let (returned_syntax_analysis_context, arguments) = parse_arguments(syntax_analysis_context);
    syntax_analysis_context = returned_syntax_analysis_context;

    // check call expression was correctly called
    match &function {
        Expression::Identifier {
            identifier_token: _,
        } => {}
        _ => {
            error!("parse_call_expression called with the function not being an Expression::IDENTIFIER.");
            return (syntax_analysis_context, None);
        }
    }

    (
        syntax_analysis_context,
        Some(Expression::Call {
            function: Box::new(function),
            arguments,
        }),
    )
}

fn parse_arguments(
    mut syntax_analysis_context: SyntaxAnalysisContext,
) -> (SyntaxAnalysisContext, Vec<Expression>) {
    debug!("Parsing arguments.");

    assert_token!(syntax_analysis_context, Token::OpeningRoundBracket, vec![]);
    let mut arguments = vec![];

    if let Some(token) = syntax_analysis_context.tokens.peek() {
        if **token != Token::ClosingRoundBracket {
            loop {
                match crate::syntax_analysis::expression::get_expression(
                    syntax_analysis_context,
                    ExpressionPrecedence::Lowest,
                ) {
                    (returned_syntax_analysis_context, Some(expression)) => {
                        arguments.push(expression);
                        syntax_analysis_context = returned_syntax_analysis_context;
                    }
                    (mut returned_syntax_analysis_context, None) => {
                        returned_syntax_analysis_context
                            .syntax_parsing_errors
                            .push("Unable to parse expression in arguments.".to_string());
                        return (returned_syntax_analysis_context, vec![]);
                    }
                }

                match syntax_analysis_context.tokens.peek() {
                    Some(token) => match token {
                        Token::ClosingRoundBracket => break,
                        Token::Comma => {
                            syntax_analysis_context.tokens.next();
                        }
                        _ => {
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

    assert_token!(syntax_analysis_context, Token::ClosingRoundBracket, vec![]);
    (syntax_analysis_context, arguments)
}

#[cfg(test)]
mod tests;
