use rstest::rstest;

#[rstest(
    code,
    snapshot_name,
    case("!5", "test_evaluator_unknown_operator_prefix_nodes_case3")
)]
fn test_evaluator_unknown_operator_prefix_nodes(code: &str, snapshot_name: &str) {
    assert_expected_returned_object!(code, snapshot_name);
}
