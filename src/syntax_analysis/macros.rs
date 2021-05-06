macro_rules! semicolon {
    ($syntax_analysis_context:expr) => {
        match $syntax_analysis_context.tokens.peek() {
            Some(token) => {
                if **token == Token::SemiColon {
                    trace!("Ignoring expression's semi colon.");
                    $syntax_analysis_context.tokens.next();
                }
            }
            None => {}
        }
    };
}

macro_rules! assert_token {
    ($syntax_analysis_context:expr, $expect_token:expr, $failure_returning:expr) => {
        match $syntax_analysis_context.tokens.next() {
            Some(token) => {
                if *token != $expect_token {
                    $syntax_analysis_context
                        .syntax_parsing_errors
                        .push(format!("Syntax error : Expected a {:?}.", $expect_token));
                    return ($syntax_analysis_context, $failure_returning);
                }
            }
            None => {
                return ($syntax_analysis_context, $failure_returning);
            }
        }
    };
}
