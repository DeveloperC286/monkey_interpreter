use rstest::rstest;

#[rstest(
    code,
    snapshot_name,
    case("\"this is a string\"", "test_evaluator_string_nodes_case1"),
    case("\"line1\nline2\n\"", "test_evaluator_string_nodes_case2")
)]
fn test_evaluator_string_nodes(code: &str, snapshot_name: &str) {
    assert_expected_returned_object!(code, snapshot_name);
}