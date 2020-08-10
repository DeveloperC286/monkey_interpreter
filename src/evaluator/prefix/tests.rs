use insta::assert_json_snapshot;
use rstest::rstest;

#[rstest(
code,
snapshot_name,
case(
"!TRUE".to_string(),
"test_evaluator_not_prefix_nodes_case1"
),
case(
"!FALSE".to_string(),
"test_evaluator_not_prefix_nodes_case2"
),
case(
"!!FALSE".to_string(),
"test_evaluator_not_prefix_nodes_case3"
),
)]
fn test_evaluator_not_prefix_nodes(code: String, snapshot_name: &str) {
    assert_expected_returned_object!(code, snapshot_name);
}

#[rstest(
code,
snapshot_name,
case(
"-5".to_string(),
"test_evaluator_minus_prefix_nodes_case1"
),
case(
"-10".to_string(),
"test_evaluator_minus_prefix_nodes_case2"
)
)]
fn test_evaluator_minus_prefix_nodes(code: String, snapshot_name: &str) {
    assert_expected_returned_object!(code, snapshot_name);
}
