macro_rules! consume_expression {
    ($syntax_analysis_context:expr) => {
        match crate::syntax_analysis::expression::get_expression(
            $syntax_analysis_context,
            ExpressionPrecedence::Lowest,
        ) {
            (returned_syntax_analysis_context, Some(expression)) => {
                $syntax_analysis_context = returned_syntax_analysis_context;
                expression
            }
            (returned_syntax_analysis_context, None) => {
                return (returned_syntax_analysis_context, None);
            }
        }
    };
}
