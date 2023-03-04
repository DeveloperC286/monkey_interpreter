macro_rules! assert_lexical_analysis_error {
    ($code:expr, $snapshot_name:expr) => {
        // When
        let error = crate::lexical_analysis::LexicalAnalysis::from($code);

        // Then
        assert!(error.is_err());
        insta::assert_debug_snapshot!(
            format!("test_{}_lexical_analysis_error", $snapshot_name),
            error
        );
    };
}
