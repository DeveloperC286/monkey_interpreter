use crate::lexical_analysis::model::token::Token;
use crate::syntax_analysis::model::abstract_syntax_tree::syntax_tree_node::{
    Statement, SyntaxTreeNode,
};
use crate::syntax_analysis::model::expression_precedence::ExpressionPrecedence;
use crate::syntax_analysis::model::syntax_analysis_context::SyntaxAnalysisContext;

#[macro_use]
mod macros;

pub(crate) fn parse_return_statement(
    mut syntax_analysis_context: SyntaxAnalysisContext,
) -> (SyntaxAnalysisContext, Option<SyntaxTreeNode>) {
    debug!("Parsing a return statement.");

    assert_token!(syntax_analysis_context, Token::Return, None);
    let expression = consume_expression!(syntax_analysis_context);
    semicolon!(syntax_analysis_context);

    (
        syntax_analysis_context,
        Some(SyntaxTreeNode::Statement {
            statement: Statement::Return {
                expression: Box::new(expression),
            },
        }),
    )
}

pub(crate) fn parse_let_statement(
    mut syntax_analysis_context: SyntaxAnalysisContext,
) -> (SyntaxAnalysisContext, Option<SyntaxTreeNode>) {
    debug!("Parsing a let statement.");

    assert_token!(syntax_analysis_context, Token::Let, None);
    let identifier_token = match syntax_analysis_context.tokens.next() {
        Some(token) => match token {
            Token::Identifier { literal: _ } => token,
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
    assert_token!(syntax_analysis_context, Token::Assign, None);
    let expression = consume_expression!(syntax_analysis_context);

    semicolon!(syntax_analysis_context);

    (
        syntax_analysis_context,
        Some(SyntaxTreeNode::Statement {
            statement: Statement::Let {
                identifier_token: identifier_token.clone(),
                expression: Box::new(expression),
            },
        }),
    )
}

#[cfg(test)]
mod tests;
