macro_rules! semicolon {
    ($iterator:expr) => {
        match $iterator.peek() {
            Some(token) => {
                if **token == Token::SEMI_COLON {
                    trace!("Ignoring expression's semi colon.");
                    $iterator.next();
                }
            }
            None => {}
        }
    };
}

macro_rules! assert_token {
    ($iterator:expr, $syntax_parsing_errors:expr, $expect_token:expr, $failure_returning:expr) => {
        match $iterator.next() {
            Some(token) => {
                if *token != $expect_token {
                    $syntax_parsing_errors
                        .push(format!("Syntax error : Expected a {:?}.", $expect_token));
                    return ($iterator, $syntax_parsing_errors, $failure_returning);
                }
            }
            None => {
                return ($iterator, $syntax_parsing_errors, $failure_returning);
            }
        }
    };
}

#[allow(unused_macros)]
macro_rules! assert_expected_returned_abstract_syntax_tree {
    ($tokens:expr, $snapshot_name:expr) => {
        assert_json_snapshot!(
            $snapshot_name,
            crate::syntax_analysis::get_abstract_syntax_tree($tokens)
        );
    };
}
