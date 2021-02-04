use crate::lexical_analysis::model::token::Token;
use crate::syntax_analysis::model::abstract_syntax_tree::syntax_tree_node::{
    Statement, SyntaxTreeNode,
};
use crate::syntax_analysis::model::expression_precedence::ExpressionPrecedence;
use crate::syntax_analysis::model::syntax_analysis_context::SyntaxAnalysisContext;

#[macro_use]
mod macros;

pub fn parse_return_statement(
    mut syntax_analysis_context: SyntaxAnalysisContext,
) -> (SyntaxAnalysisContext, Option<SyntaxTreeNode>) {
    debug!("Parsing a return statement.");

    assert_token!(syntax_analysis_context, Token::RETURN, None);
    let expression = consume_expression!(syntax_analysis_context);
    semicolon!(syntax_analysis_context);

    (
        syntax_analysis_context,
        Some(SyntaxTreeNode::STATEMENT {
            statement: Statement::RETURN {
                expression: Box::new(expression),
            },
        }),
    )
}

pub fn parse_let_statement(
    mut syntax_analysis_context: SyntaxAnalysisContext,
) -> (SyntaxAnalysisContext, Option<SyntaxTreeNode>) {
    debug!("Parsing a let statement.");

    assert_token!(syntax_analysis_context, Token::LET, None);
    let identifier_token = match syntax_analysis_context.tokens.next() {
        Some(token) => match token {
            Token::IDENTIFIER { literal: _ } => token,
            _ => {
                syntax_analysis_context
                    .syntax_parsing_errors
                    .push("Syntax error : Expected a IDENTIFIER.".to_string());
                return (syntax_analysis_context, None);
            }
        },
        None => {
            return (syntax_analysis_context, None);
        }
    };
    assert_token!(syntax_analysis_context, Token::ASSIGN, None);
    let expression = consume_expression!(syntax_analysis_context);

    semicolon!(syntax_analysis_context);

    (
        syntax_analysis_context,
        Some(SyntaxTreeNode::STATEMENT {
            statement: Statement::LET {
                identifier_token: identifier_token.clone(),
                expression: Box::new(expression),
            },
        }),
    )
}

#[cfg(test)]
mod tests;
