use insta::assert_json_snapshot;
use rstest::rstest;

#[rstest(
code,
snapshot_name,
case(
"fn(x,y){x+y;}".to_string(),
"test_syntax_analysis_for_function_expression_case1"
),
case(
"fn(){}".to_string(),
"test_syntax_analysis_for_function_expression_case2"
),
case(
"fn(x){}".to_string(),
"test_syntax_analysis_for_function_expression_case3"
),
)]
fn test_syntax_analysis_for_function_expression(code: String, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(code, snapshot_name);
}
