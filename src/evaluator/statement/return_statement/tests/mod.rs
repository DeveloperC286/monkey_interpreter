use rstest::rstest;

#[rstest(
    code,
    snapshot_name,
    case("return 10;", "test_evaluator_return_nodes_case1"),
    case("return TRUE; 9;", "test_evaluator_return_nodes_case2"),
    case("return 3 * 4; FALSE;", "test_evaluator_return_nodes_case3"),
    case("13; return 8 / 2; 0;", "test_evaluator_return_nodes_case4")
)]
fn test_evaluator_return_nodes(code: &str, snapshot_name: &str) {
    assert_expected_returned_object!(code, snapshot_name);
}
