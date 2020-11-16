use insta::assert_json_snapshot;
use rstest::rstest;

#[rstest(
    code,
    snapshot_name,
    case("if (true) { 10 }", "test_evaluator_if_nodes_case1"),
    case("if (false) { 10 }", "test_evaluator_if_nodes_case2"),
    case("if (1) { 1 }", "test_evaluator_if_nodes_case3"),
    case("if (1 > 2) { 5 }", "test_evaluator_if_nodes_case4"),
    case("if (1 < 2) { 4 } else { true }", "test_evaluator_if_nodes_case5"),
    case("if (4 > 6) { true } else { false }", "test_evaluator_if_nodes_case6"),
    case("if (false) { 85 } else { 58 }", "test_evaluator_if_nodes_case7")
)]
fn test_evaluator_if_nodes(code: &str, snapshot_name: &str) {
    assert_expected_returned_object!(code, snapshot_name);
}
