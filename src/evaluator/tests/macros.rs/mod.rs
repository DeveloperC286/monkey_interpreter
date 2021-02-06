macro_rules! assert_expected_returned_object {
    ($code:expr, $snapshot_name:expr) => {
        insta::assert_debug_snapshot!(
            $snapshot_name,
            crate::evaluator::evaluate(
                crate::evaluator::model::evaluator_context::EvaluatorContext::new(),
                crate::syntax_analysis::get_abstract_syntax_tree(
                    crate::lexical_analysis::get_tokens($code)
                )
            )
        );
    };
}
