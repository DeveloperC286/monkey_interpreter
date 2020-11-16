use crate::lexical_analysis::token::Token;
use crate::syntax_analysis::abstract_syntax_tree::syntax_tree_node::{
    Expression, ExpressionPrecedence,
};
use crate::syntax_analysis::syntax_analysis_context::SyntaxAnalysisContext;

pub fn parse_grouped_expression(
    mut syntax_analysis_context: SyntaxAnalysisContext,
) -> (SyntaxAnalysisContext, Option<Expression>) {
    debug!("Parsing a grouped expression.");

    assert_token!(syntax_analysis_context, Token::OPENING_ROUND_BRACKET, None);
    let (returned_syntax_analysis_context, grouped_expression) =
        crate::syntax_analysis::expression::get_expression(
            syntax_analysis_context,
            ExpressionPrecedence::LOWEST,
        );
    syntax_analysis_context = returned_syntax_analysis_context;
    assert_token!(syntax_analysis_context, Token::CLOSING_ROUND_BRACKET, None);

    (syntax_analysis_context, grouped_expression)
}

#[cfg(test)]
mod tests;
