macro_rules! assert_expected_returned_object {
    ($code:expr, $snapshot_name:expr) => {
        let mut evaluator = crate::evaluator::Evaluator::new();

        insta::assert_debug_snapshot!(
            $snapshot_name,
            evaluator.evaluate(crate::syntax_analysis::SyntaxAnalysis::from(
                crate::lexical_analysis::LexicalAnalysis::from($code).unwrap()
            ))
        );
    };
}
