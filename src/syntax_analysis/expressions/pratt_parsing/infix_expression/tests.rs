use insta::assert_json_snapshot;
use rstest::rstest;

#[rstest(
code,
snapshot_name,
case(
"15 + 5;".to_string(),
"test_syntax_analysis_for_infix_expression_case1"
),
case(
"6 - 6;".to_string(),
"test_syntax_analysis_for_infix_expression_case2"
),
case(
"3 * 4;".to_string(),
"test_syntax_analysis_for_infix_expression_case3"
),
case(
"5 / 10;".to_string(),
"test_syntax_analysis_for_infix_expression_case4"
),
case(
"-5 * 10;".to_string(),
"test_syntax_analysis_for_infix_expression_case5"
),
case(
"5 - 10 * 10;".to_string(),
"test_syntax_analysis_for_infix_expression_case6"
),
case(
"10 / 5 + 1;".to_string(),
"test_syntax_analysis_for_infix_expression_case7"
),
case(
"10 > 5 == true".to_string(),
"test_syntax_analysis_for_infix_expression_case8"
),
case(
"10 < 5 != false".to_string(),
"test_syntax_analysis_for_infix_expression_case9"
),
)]
fn test_syntax_analysis_for_infix_expression(code: String, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(code, snapshot_name);
}
