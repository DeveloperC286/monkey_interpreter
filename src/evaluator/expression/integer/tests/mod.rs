use rstest::rstest;

#[rstest(code, snapshot_name, case("5", "test_evaluator_integer_nodes_case1"))]
fn test_evaluator_integer_nodes(code: &str, snapshot_name: &str) {
    assert_expected_returned_object!(code, snapshot_name);
}
