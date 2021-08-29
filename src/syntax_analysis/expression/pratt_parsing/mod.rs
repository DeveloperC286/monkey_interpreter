use std::cmp::Ordering;

use crate::lexical_analysis::model::token::Token;
use crate::syntax_analysis::model::abstract_syntax_tree::syntax_tree_node::Expression;
use crate::syntax_analysis::model::expression_precedence::ExpressionPrecedence;
use crate::syntax_analysis::model::syntax_analysis_context::SyntaxAnalysisContext;

mod call_expression;
mod infix_expression;

pub fn pratt_parsing(
    mut syntax_analysis_context: SyntaxAnalysisContext,
    mut expression: Option<Expression>,
    expression_precedence: ExpressionPrecedence,
) -> (SyntaxAnalysisContext, Option<Expression>) {
    while let Some(token) = syntax_analysis_context.tokens.peek() {
        if **token == Token::SemiColon {
            break;
        }

        //if expression_precedence.
        let expression_precedence_comparison = expression_precedence.partial_cmp(
            &crate::syntax_analysis::model::expression_precedence::get_current_expression_precedence(
                token,
            ),
        );
        if expression_precedence_comparison != Some(Ordering::Less) {
            break;
        }

        match token {
            Token::Plus
            | Token::Minus
            | Token::Divide
            | Token::Multiply
            | Token::Equals
            | Token::NotEquals
            | Token::LesserThan
            | Token::GreaterThan => {
                let (returned_syntax_analysis_context, returned_expression) =
                    infix_expression::parse_infix_expression(
                        syntax_analysis_context,
                        expression.unwrap(),
                    );
                syntax_analysis_context = returned_syntax_analysis_context;
                expression = returned_expression;
            }
            Token::OpeningRoundBracket => {
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
