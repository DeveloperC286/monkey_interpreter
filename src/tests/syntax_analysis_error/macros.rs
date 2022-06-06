macro_rules! assert_syntax_analysis_error {
    ($code:expr, $snapshot_name:expr) => {
        // When
        let error = crate::syntax_analysis::SyntaxAnalysis::from(
            crate::lexical_analysis::LexicalAnalysis::from($code).unwrap(),
        );

        // Then
        assert!(error.is_err());
        insta::assert_debug_snapshot!(format!("test_{}_syntax_analysis", $snapshot_name), error);
    };
}
