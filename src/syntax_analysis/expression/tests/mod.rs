use rstest::rstest;

#[rstest(
    code,
    snapshot_name,
    case("!15", "test_syntax_analysis_for_prefix_expressions_case1")
)]
fn test_syntax_analysis_for_prefix_expressions(code: &str, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(code, snapshot_name)
}
