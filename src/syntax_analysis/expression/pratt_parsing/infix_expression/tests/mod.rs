use rstest::rstest;

#[rstest(
    code,
    snapshot_name,
    case("10 > 5 == true", "test_syntax_analysis_for_infix_expression_case8"),
    case("10 < 5 != false", "test_syntax_analysis_for_infix_expression_case9")
)]
fn test_syntax_analysis_for_infix_expression(code: &str, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(code, snapshot_name);
}
