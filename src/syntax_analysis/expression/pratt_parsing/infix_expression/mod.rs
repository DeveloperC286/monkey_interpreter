use crate::syntax_analysis::abstract_syntax_tree::syntax_tree_node::Expression;
use crate::syntax_analysis::model::syntax_analysis_context::SyntaxAnalysisContext;

pub fn parse_infix_expression(
    mut syntax_analysis_context: SyntaxAnalysisContext,
    left_hand: Expression,
) -> (SyntaxAnalysisContext, Option<Expression>) {
    debug!("Parsing a infix expression.");

    let operator_token = match syntax_analysis_context.tokens.next() {
        Some(token) => token,
        None => return (syntax_analysis_context, None),
    };
    let precedence =
        crate::syntax_analysis::model::expression_precedence::get_current_expression_precedence(
            &operator_token,
        );

    match crate::syntax_analysis::expression::get_expression(syntax_analysis_context, precedence) {
        (returned_syntax_analysis_context, Some(right_hand)) => (
            returned_syntax_analysis_context,
            Some(Expression::INFIX {
                left_hand: Box::new(left_hand),
                operator_token: operator_token.clone(),
                right_hand: Box::new(right_hand),
            }),
        ),
        (returned_syntax_analysis_context, None) => (returned_syntax_analysis_context, None),
    }
}

#[cfg(test)]
mod tests;
