use insta::assert_json_snapshot;
use rstest::rstest;

#[rstest(
code,
snapshot_name,
case(
"".to_string(),
"test_evaluator_input_edgecases_case1"
),
)]
fn test_evaluator_input_edgecases(code: String, snapshot_name: &str) {
    assert_expected_returned_object!(code, snapshot_name);
}
