use rstest::rstest;

#[rstest(
    code,
    snapshot_name,
    case("add(5, 12)", "test_syntax_analysis_for_call_expression_case1"),
    case("add(3, 7 * 2);", "test_syntax_analysis_for_call_expression_case2")
)]
fn test_syntax_analysis_for_call_expression(code: &str, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(code, snapshot_name);
}
