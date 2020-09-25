use insta::assert_json_snapshot;
use rstest::rstest;

#[rstest(
    code,
    snapshot_name,
    case("true", "test_evaluator_boolean_nodes_case1"),
    case("false", "test_evaluator_boolean_nodes_case2")
)]
fn test_evaluator_boolean_nodes(code: &str, snapshot_name: &str) {
    assert_expected_returned_object!(code, snapshot_name);
}
