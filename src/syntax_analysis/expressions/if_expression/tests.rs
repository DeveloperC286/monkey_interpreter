use insta::assert_json_snapshot;
use rstest::rstest;

#[rstest(
    code,
    snapshot_name,
    case(
        "if (x < y) { x }".to_string(),
        "test_syntax_analysis_for_if_expression_case1"
    ),
    case(
        "if (x < y) { x } else { y }".to_string(),
        "test_syntax_analysis_for_if_expression_case2"
    ),
)]
fn test_syntax_analysis_for_if_expression(code: String, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(code, snapshot_name);
}
