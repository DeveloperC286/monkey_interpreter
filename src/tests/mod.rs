macro_rules! assert_expected_returned_object {
    ($code:expr, $snapshot_name:expr) => {
        insta::assert_debug_snapshot!(
            $snapshot_name,
            crate::evaluator::evaluate(
                crate::evaluator::evaluator_context::EvaluatorContext::new(),
                crate::syntax_analysis::get_abstract_syntax_tree(
                    crate::lexical_analysis::get_tokens($code)
                )
            )
        );
    };
}

macro_rules! assert_expected_returned_abstract_syntax_tree {
    ($code:expr, $snapshot_name:expr) => {
        assert_json_snapshot!(
            $snapshot_name,
            crate::syntax_analysis::get_abstract_syntax_tree(crate::lexical_analysis::get_tokens(
                $code
            ))
        );
    };
}

macro_rules! assert_expected_returned_tokens {
    ($code:expr, $snapshot_name:expr) => {
        assert_json_snapshot!($snapshot_name, crate::lexical_analysis::get_tokens($code));
    };
}
