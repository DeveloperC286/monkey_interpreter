use insta::assert_json_snapshot;
use rstest::rstest;

#[rstest(
code,
snapshot_name,
case(
"return 5;".to_string(),
"test_syntax_analysis_for_return_statements_case1"
),
case(
"return 5; return x;".to_string(),
"test_syntax_analysis_for_return_statements_case2"
),
)]
fn test_syntax_analysis_for_return_statements(code: String, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(code, snapshot_name);
}

#[rstest(
code,
snapshot_name,
case(
"let x = 5;".to_string(),
"test_syntax_analysis_for_let_statements_case1"
),
case(
"let x = 5;let z = 7 + 10;".to_string(),
"test_syntax_analysis_for_let_statements_case2"
),
)]
fn test_syntax_analysis_for_let_statements(code: String, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(code, snapshot_name);
}

#[rstest(
code,
snapshot_name,
case(
"let x".to_string(),
"test_syntax_analysis_for_let_statements_syntax_errors_case1"
),
case(
"let".to_string(),
"test_syntax_analysis_for_let_statements_syntax_errors_case2"
),
)]
fn test_syntax_analysis_for_let_statements_syntax_errors(code: String, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(code, snapshot_name);
}

#[rstest(
code,
snapshot_name,
case(
"let x = 5; return 5;".to_string(),
"test_syntax_analysis_for_combined_statements_case1"
),
)]
fn test_syntax_analysis_for_combined_statements(code: String, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(code, snapshot_name)
}
