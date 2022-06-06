use rstest::rstest;

#[rstest(
    code,
    snapshot_name,
    case("(1 > 2) == 5", "test_evaluator_infix_type_mismatch_expressions_case1"),
    case(
        "\"this is a string\" == 3",
        "test_evaluator_infix_type_mismatch_expressions_case5"
    ),
    case(
        "82 != \"this is a string\"",
        "test_evaluator_infix_type_mismatch_expressions_case6"
    )
)]
fn test_evaluator_infix_type_mismatch_expressions(code: &str, snapshot_name: &str) {
    assert_expected_returned_object!(code, snapshot_name);
}
