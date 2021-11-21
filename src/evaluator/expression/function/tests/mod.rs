use rstest::rstest;

#[rstest(
    code,
    snapshot_name,
    case("fn(x) { x + 3 }", "test_evaluator_function_nodes_case1"),
    case("fn(x,y) { x + y; };", "test_evaluator_function_nodes_case2")
)]
fn test_evaluator_function_nodes(code: &str, snapshot_name: &str) {
    assert_expected_returned_object!(code, snapshot_name);
}
