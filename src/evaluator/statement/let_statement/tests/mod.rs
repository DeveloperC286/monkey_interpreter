use rstest::rstest;

#[rstest(
    code,
    snapshot_name,
    case("let a = 5; a;", "test_evaluator_let_nodes_case1"),
    case("let b = 5 + 5; b;", "test_evaluator_let_nodes_case2"),
    case("let a = 6; let b = a; b;", "test_evaluator_let_nodes_case3"),
    case("let a = 6; let b = 3; a*b;", "test_evaluator_let_nodes_case4"),
    case(
        "let b = true; if (b) { 10 } else { 1 }",
        "test_evaluator_let_nodes_case5"
    ),
    case(
        "let sentence = \"line1\\nline2 line22\\n\"; sentence;",
        "test_evaluator_let_nodes_case6"
    )
)]
fn test_evaluator_let_nodes(code: &str, snapshot_name: &str) {
    assert_expected_returned_object!(code, snapshot_name);
}
