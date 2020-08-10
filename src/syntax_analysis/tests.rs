use insta::assert_json_snapshot;
use rstest::rstest;

#[rstest(
code,
snapshot_name,
case(
"".to_string(),
"test_syntax_analysis_tokens_input_edgecases_case1"
),
case(
";".to_string(),
"test_syntax_analysis_tokens_input_edgecases_case2"
),
)]
fn test_syntax_analysis_tokens_input_edgecases(code: String, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(code, snapshot_name)
}
