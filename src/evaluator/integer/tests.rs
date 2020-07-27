use insta::assert_json_snapshot;
use rstest::rstest;

use super::*;

#[rstest(
    code,
    snapshot_name,
    case(
        "5".to_string(),
        "test_evaluator_integer_nodes_case1"
    ),
)]
fn test_evaluator_integer_nodes(code: String, snapshot_name: &str) {
    assert_expected_returned_object!(code, snapshot_name);
}
