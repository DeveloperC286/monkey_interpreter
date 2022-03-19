use rstest::rstest;

#[rstest(
    code,
    snapshot_name,
    case("let x", "test_syntax_analysis_for_let_statements_syntax_errors_case1"),
    case("let", "test_syntax_analysis_for_let_statements_syntax_errors_case2")
)]
fn test_syntax_analysis_for_let_statements_syntax_errors(code: &str, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(code, snapshot_name);
}
