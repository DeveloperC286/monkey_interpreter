use crate::lexical_analysis::model::token::Token;
use crate::syntax_analysis::expression::utilities::parse_block;
use crate::syntax_analysis::model::abstract_syntax_tree::syntax_tree_node::Expression;
use crate::syntax_analysis::model::expression_precedence::ExpressionPrecedence;
use crate::syntax_analysis::model::syntax_analysis_context::SyntaxAnalysisContext;

pub fn parse_if_expression(
    mut syntax_analysis_context: SyntaxAnalysisContext,
) -> (SyntaxAnalysisContext, Option<Expression>) {
    debug!("Parsing a if expression.");

    // parse if expression
    assert_token!(syntax_analysis_context, Token::If, None);
    assert_token!(syntax_analysis_context, Token::OpeningRoundBracket, None);
    let (returned_syntax_analysis_context, condition_option) =
        crate::syntax_analysis::expression::get_expression(
            syntax_analysis_context,
            ExpressionPrecedence::Lowest,
        );
    syntax_analysis_context = returned_syntax_analysis_context;
    assert_token!(syntax_analysis_context, Token::ClosingRoundBracket, None);
    let (returned_syntax_analysis_context, consequence_option) =
        parse_block(syntax_analysis_context);
    syntax_analysis_context = returned_syntax_analysis_context;
    let mut alternative = None;

    if let Some(token) = syntax_analysis_context.tokens.peek() {
        if **token == Token::Else {
            assert_token!(syntax_analysis_context, Token::Else, None);
            let (returned_syntax_analysis_context, returned_alternative) =
                parse_block(syntax_analysis_context);
            alternative = returned_alternative;
            syntax_analysis_context = returned_syntax_analysis_context;
        }
    }

    // check if expression was parsed correctly
    let condition = match condition_option {
        Some(condition) => condition,
        None => {
            return (syntax_analysis_context, None);
        }
    };

    let consequence = match consequence_option {
        Some(consequence) => consequence,
        None => {
            return (syntax_analysis_context, None);
        }
    };

    (
        syntax_analysis_context,
        Some(Expression::IF {
            condition: Box::new(condition),
            consequence: Box::new(consequence),
            alternative: Box::new(alternative),
        }),
    )
}

#[cfg(test)]
mod tests;
