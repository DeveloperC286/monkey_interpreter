macro_rules! consume_expression {
    ($iterator:expr, $syntax_parsing_errors:expr) => {
        match crate::syntax_analysis::expressions::get_expression(
            $iterator,
            $syntax_parsing_errors,
            ExpressionPrecedence::LOWEST,
        ) {
            (returned_iterator, returned_syntax_parsing_errors, Some(expression)) => {
                $iterator = returned_iterator;
                $syntax_parsing_errors = returned_syntax_parsing_errors;
                expression
            }
            (returned_iterator, returned_syntax_parsing_errors, None) => {
                return (returned_iterator, returned_syntax_parsing_errors, None);
            }
        };
    };
}
