use rstest::rstest;

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
    case("!15", "test_syntax_analysis_for_prefix_expressions_case1")
)]
fn test_syntax_analysis_for_prefix_expressions(code: &str, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(code, snapshot_name)
}

#[rstest(
    code,
    snapshot_name,
    case("temp;", "test_syntax_analysis_for_identifier_expressions_case1"),
    case("varX", "test_syntax_analysis_for_identifier_expressions_case2")
)]
fn test_syntax_analysis_for_identifier_expressions(code: &str, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(code, snapshot_name)
}

#[rstest(
    code,
    snapshot_name,
    case(
        "let name = \"firstName\" + \"lastName\";",
        "test_syntax_analysis_for_string_expressions_case3"
    )
)]
fn test_syntax_analysis_for_string_expressions(code: &str, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(code, snapshot_name)
}
