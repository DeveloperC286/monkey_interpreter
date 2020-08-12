use insta::assert_json_snapshot;
use rstest::rstest;

#[rstest(
    code,
    snapshot_name,
    case(
        "return 10;".to_string(),
        "test_evaluator_return_nodes_case1"
    ),
    case(
        "return TRUE; 9;".to_string(),
        "test_evaluator_return_nodes_case2"
    ),
    case(
        "return 3 * 4; FALSE;".to_string(),
        "test_evaluator_return_nodes_case3"
    ),
    case(
        "13; return 8 / 2; 0;".to_string(),
        "test_evaluator_return_nodes_case4"
    ),
    case(
        "if (10 > 1) { if (10 > 1) { return TRUE;} return FALSE;}".to_string(),
        "test_evaluator_return_nodes_case5"
    ),
)]
fn test_evaluator_return_nodes(code: String, snapshot_name: &str) {
    assert_expected_returned_object!(code, snapshot_name);
}
