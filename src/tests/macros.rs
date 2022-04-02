macro_rules! assert_lexical_analysis {
    ($code:expr, $snapshot_name:expr) => {
        // When
        let tokens = crate::lexical_analysis::LexicalAnalysis::from($code).unwrap();

        // Then
        insta::assert_debug_snapshot!(format!("test_{}_lexical_analysis", $snapshot_name), tokens);
    };
}

macro_rules! assert_lexical_analysis_error {
    ($code:expr, $snapshot_name:expr) => {
        // When
        let error = crate::lexical_analysis::LexicalAnalysis::from($code);

        // Then
        assert!(error.is_err());
        insta::assert_debug_snapshot!(format!("test_{}_lexical_analysis", $snapshot_name), error);
    };
}

macro_rules! assert_syntax_analysis {
    ($code:expr, $snapshot_name:expr) => {
        // When
        let abstract_syntax_tree = crate::syntax_analysis::SyntaxAnalysis::from(
            crate::lexical_analysis::LexicalAnalysis::from($code).unwrap(),
        )
        .unwrap();

        // Then
        insta::assert_debug_snapshot!(
            format!("test_{}_syntax_analysis", $snapshot_name),
            abstract_syntax_tree
        );
    };
}

macro_rules! assert_evaluator {
    ($code:expr, $snapshot_name:expr) => {
        // When
        let mut evaluator = crate::evaluator::Evaluator::new();
        let evaluation = evaluator
            .evaluate(
                crate::syntax_analysis::SyntaxAnalysis::from(
                    crate::lexical_analysis::LexicalAnalysis::from($code).unwrap(),
                )
                .unwrap(),
            )
            .unwrap();

        // Then
        insta::assert_debug_snapshot!(format!("test_{}_evaluator", $snapshot_name), evaluation);
    };
}

macro_rules! assert_environment {
    ($code:expr, $snapshot_name:expr) => {
        // When
        let mut evaluator = crate::evaluator::Evaluator::new();
        let _evaluation = evaluator
            .evaluate(
                crate::syntax_analysis::SyntaxAnalysis::from(
                    crate::lexical_analysis::LexicalAnalysis::from($code).unwrap(),
                )
                .unwrap(),
            )
            .unwrap();

        // Then
        insta::assert_debug_snapshot!(format!("test_{}_environment", $snapshot_name), evaluator);
    };
}
