macro_rules! consume_expression {
    ($syntax_analysis_context:expr) => {
        match crate::syntax_analysis::expressions::get_expression(
            $syntax_analysis_context,
            ExpressionPrecedence::LOWEST,
        ) {
            (returned_syntax_analysis_context, Some(expression)) => {
                $syntax_analysis_context = returned_syntax_analysis_context;
                expression
            }
            (returned_syntax_analysis_context, None) => {
                return (returned_syntax_analysis_context, None);
            }
        };
    };
}
