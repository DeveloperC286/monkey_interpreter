use insta::assert_json_snapshot;
use rstest::rstest;

#[rstest(
code,
snapshot_name,
case(
"true;".to_string(),
"test_syntax_analysis_for_boolean_expression_case1"
),
case(
"false".to_string(),
"test_syntax_analysis_for_boolean_expression_case2"
),
)]
fn test_syntax_analysis_for_boolean_expression(code: String, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(code, snapshot_name)
}

#[rstest(
code,
snapshot_name,
case(
"!".to_string(),
"test_syntax_analysis_for_prefix_expression_syntax_errors_case1"
),
case(
"-;".to_string(),
"test_syntax_analysis_for_prefix_expression_syntax_errors_case2"
),
)]
fn test_syntax_analysis_for_prefix_expression_syntax_errors(code: String, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(code, snapshot_name)
}

#[rstest(
code,
snapshot_name,
case(
"!15".to_string(),
"test_syntax_analysis_for_prefix_expression_case1"
),
case(
"-3;".to_string(),
"test_syntax_analysis_for_prefix_expression_case2"
),
case(
"!FALSE".to_string(),
"test_syntax_analysis_for_prefix_expression_case3"
),
)]
fn test_syntax_analysis_for_prefix_expression(code: String, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(code, snapshot_name)
}

#[rstest(
code,
snapshot_name,
case(
"temp;".to_string(),
"test_syntax_analysis_for_identifier_expression_case1"
),
case(
"varX".to_string(),
"test_syntax_analysis_for_identifier_expression_case2"
),
)]
fn test_syntax_analysis_for_identifier_expression(code: String, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(code, snapshot_name)
}

#[rstest(
code,
snapshot_name,
case(
"5;".to_string(),
"test_syntax_analysis_for_integer_expression_case1"
),
case(
"12".to_string(),
"test_syntax_analysis_for_integer_expression_case2"
),
)]
fn test_syntax_analysis_for_integer_expression(code: String, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(code, snapshot_name)
}
