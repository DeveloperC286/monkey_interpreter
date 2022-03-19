use rstest::rstest;

#[rstest(
    code,
    snapshot_name,
    case(
        "\"name\" == \"name\"",
        "test_evaluator_infix_boolean_expressions_case21"
    ),
    case(
        "\"this is a string\" == \"this is a string\"",
        "test_evaluator_infix_boolean_expressions_case22"
    ),
    case(
        "\"first\" != \"last\"",
        "test_evaluator_infix_boolean_expressions_case23"
    ),
    case(
        "\"string123\" != \"\\tstring123\"",
        "test_evaluator_infix_boolean_expressions_case24"
    ),
    case(
        "\"string123\" != \"string123\"",
        "test_evaluator_infix_boolean_expressions_case25"
    ),
    case(
        "\"name\" == \"name123\"",
        "test_evaluator_infix_boolean_expressions_case26"
    )
)]
fn test_evaluator_infix_boolean_expressions(code: &str, snapshot_name: &str) {
    assert_expected_returned_object!(code, snapshot_name);
}

#[rstest(
    code,
    snapshot_name,
    case("(1 > 2) == 5", "test_evaluator_infix_type_mismatch_expressions_case1"),
    case(
        "5 + TRUE; 5 + 10;",
        "test_evaluator_infix_type_mismatch_expressions_case2"
    ),
    case("FALSE - 10;", "test_evaluator_infix_type_mismatch_expressions_case3"),
    case(
        "if (10 > 1) { return TRUE + 5; } return 1;",
        "test_evaluator_infix_type_mismatch_expressions_case4"
    ),
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

#[rstest(
    code,
    snapshot_name,
    case(
        "true + TRUE",
        "test_evaluator_infix_unknown_operator_expressions_case1"
    ),
    case(
        "5 + 10; FALSE * FALSE; 7",
        "test_evaluator_infix_unknown_operator_expressions_case2"
    ),
    case(
        "if (5 < 10) { FALSE + FALSE }",
        "test_evaluator_infix_unknown_operator_expressions_case3"
    )
)]
fn test_evaluator_infix_unknown_operator_expressions(code: &str, snapshot_name: &str) {
    assert_expected_returned_object!(code, snapshot_name);
}
