use rstest::rstest;

#[rstest(
    code,
    snapshot_name,
    case(
        "let identity = fn(x) { x; }; identity(5);",
        "test_evaluator_call_nodes_case1"
    ),
    case(
        "let identity = fn(x) { return x; }; identity(5);",
        "test_evaluator_call_nodes_case2"
    ),
    case(
        "let double = fn(x) { x * 2 }; double(5);",
        "test_evaluator_call_nodes_case3"
    ),
    case(
        "let add = fn(x, y) { x + y }; add(2, 3);",
        "test_evaluator_call_nodes_case4"
    ),
    case(
        "let add = fn(x, y) { x + y }; add(1, add(2, 3));",
        "test_evaluator_call_nodes_case5"
    ),
    case("fn(y) { y }(13)", "test_evaluator_call_nodes_case6")
)]
fn test_evaluator_call_nodes(code: &str, snapshot_name: &str) {
    assert_expected_returned_object!(code, snapshot_name);
}
