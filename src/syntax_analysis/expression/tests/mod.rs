use rstest::rstest;

#[rstest(
    code,
    snapshot_name,
    case("true;", "test_syntax_analysis_for_boolean_expression_case1"),
    case("false", "test_syntax_analysis_for_boolean_expression_case2")
)]
fn test_syntax_analysis_for_boolean_expression(code: &str, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(code, snapshot_name)
}

#[rstest(
    code,
    snapshot_name,
    case("!", "test_syntax_analysis_for_prefix_expression_syntax_errors_case1"),
    case("-;", "test_syntax_analysis_for_prefix_expression_syntax_errors_case2")
)]
fn test_syntax_analysis_for_prefix_expression_syntax_errors(code: &str, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(code, snapshot_name)
}

#[rstest(
    code,
    snapshot_name,
    case("!15", "test_syntax_analysis_for_prefix_expression_case1"),
    case("-3;", "test_syntax_analysis_for_prefix_expression_case2"),
    case("!FALSE", "test_syntax_analysis_for_prefix_expression_case3")
)]
fn test_syntax_analysis_for_prefix_expression(code: &str, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(code, snapshot_name)
}

#[rstest(
    code,
    snapshot_name,
    case("temp;", "test_syntax_analysis_for_identifier_expression_case1"),
    case("varX", "test_syntax_analysis_for_identifier_expression_case2")
)]
fn test_syntax_analysis_for_identifier_expression(code: &str, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(code, snapshot_name)
}

#[rstest(
    code,
    snapshot_name,
    case("5;", "test_syntax_analysis_for_integer_expression_case1"),
    case("12", "test_syntax_analysis_for_integer_expression_case2")
)]
fn test_syntax_analysis_for_integer_expression(code: &str, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(code, snapshot_name)
}
