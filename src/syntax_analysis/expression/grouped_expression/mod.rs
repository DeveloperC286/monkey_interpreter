use crate::lexical_analysis::model::token::Token;
use crate::syntax_analysis::model::abstract_syntax_tree::syntax_tree_node::Expression;
use crate::syntax_analysis::model::expression_precedence::ExpressionPrecedence;
use crate::syntax_analysis::model::syntax_analysis_context::SyntaxAnalysisContext;

pub fn parse_grouped_expression(
    mut syntax_analysis_context: SyntaxAnalysisContext,
) -> (SyntaxAnalysisContext, Option<Expression>) {
    debug!("Parsing a grouped expression.");

    assert_token!(syntax_analysis_context, Token::OpeningRoundBracket, None);
    let (returned_syntax_analysis_context, grouped_expression) =
        crate::syntax_analysis::expression::get_expression(
            syntax_analysis_context,
            ExpressionPrecedence::Lowest,
        );
    syntax_analysis_context = returned_syntax_analysis_context;
    assert_token!(syntax_analysis_context, Token::ClosingRoundBracket, None);

    (syntax_analysis_context, grouped_expression)
}

#[cfg(test)]
mod tests;
