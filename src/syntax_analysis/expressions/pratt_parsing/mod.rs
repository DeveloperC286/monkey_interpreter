use std::cmp::Ordering;

use crate::lexical_analysis::token::Token;
use crate::syntax_analysis::abstract_syntax_tree::syntax_tree_node::{
    Expression, ExpressionPrecedence,
};
use crate::syntax_analysis::syntax_analysis_context::SyntaxAnalysisContext;

mod call_expression;
mod infix_expression;

pub fn pratt_parsing(
    mut syntax_analysis_context: SyntaxAnalysisContext,
    mut expression: Option<Expression>,
    expression_precedence: ExpressionPrecedence,
) -> (SyntaxAnalysisContext, Option<Expression>) {
    while let Some(token) = syntax_analysis_context.tokens.peek() {
        if **token == Token::SEMI_COLON {
            break;
        }

        //if expression_precedence.
        let expression_precedence_comparison = expression_precedence.partial_cmp(
            &crate::syntax_analysis::expression_precedence::get_current_expression_precedence(
                &token,
            ),
        );
        if expression_precedence_comparison != Some(Ordering::Less) {
            break;
        }

        match token {
            Token::PLUS
            | Token::MINUS
            | Token::DIVIDE
            | Token::MULTIPLY
            | Token::EQUALS
            | Token::NOT_EQUALS
            | Token::LESSER_THAN
            | Token::GREATER_THAN => {
                let (returned_syntax_analysis_context, returned_expression) =
                    infix_expression::parse_infix_expression(
                        syntax_analysis_context,
                        expression.unwrap(),
                    );
                syntax_analysis_context = returned_syntax_analysis_context;
                expression = returned_expression;
            }
            Token::OPENING_ROUND_BRACKET => {
                let (returned_syntax_analysis_context, returned_expression) =
                    call_expression::parse_call_expression(
                        syntax_analysis_context,
                        expression.unwrap(),
                    );
                syntax_analysis_context = returned_syntax_analysis_context;
                expression = returned_expression;
            }
            _ => {
                break;
            }
        }
    }

    (syntax_analysis_context, expression)
}
