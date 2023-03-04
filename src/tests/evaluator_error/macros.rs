macro_rules! assert_evaluation_error {
    ($code:expr, $snapshot_name:expr) => {
        // When
        let mut evaluator = crate::evaluator::Evaluator::new();
        let error = evaluator.evaluate(
            crate::syntax_analysis::SyntaxAnalysis::from(
                crate::lexical_analysis::LexicalAnalysis::from($code).unwrap(),
            )
            .unwrap(),
        );

        // Then
        assert!(error.is_err());
        insta::assert_debug_snapshot!(format!("test_{}_evaluation", $snapshot_name), error);
    };
}
