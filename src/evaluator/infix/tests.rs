use insta::assert_json_snapshot;
use rstest::rstest;

#[rstest(
    code,
    snapshot_name,
    case(
        "5 + 5".to_string(),
        "test_evaluator_infix_integer_expressions_case1"
    ),
    case(
        "5 + 5  + 10".to_string(),
        "test_evaluator_infix_integer_expressions_case2"
    ),
    case(
        "10 - 15".to_string(),
        "test_evaluator_infix_integer_expressions_case3"
    ),
    case(
        "-3 - -3".to_string(),
        "test_evaluator_infix_integer_expressions_case4"
    ),
    case(
        "2 * 12".to_string(),
        "test_evaluator_infix_integer_expressions_case5"
    ),
    case(
        "37 * -1".to_string(),
        "test_evaluator_infix_integer_expressions_case6"
    ),
    case(
        "20 / 10".to_string(),
        "test_evaluator_infix_integer_expressions_case7"
    ),
    case(
        "-100 / 10".to_string(),
        "test_evaluator_infix_integer_expressions_case8"
    ),
    case(
        "5 + 5 * 10".to_string(),
        "test_evaluator_infix_integer_expressions_case9"
    ),
    case(
        "(5 + 5) / 10".to_string(),
        "test_evaluator_infix_integer_expressions_case10"
    ),
)]
fn test_evaluator_infix_integer_expressions(code: String, snapshot_name: &str) {
    assert_expected_returned_object!(code, snapshot_name);
}

#[rstest(
    code,
    snapshot_name,
    case(
        "2 < 3".to_string(),
        "test_evaluator_infix_boolean_expressions_case1"
    ),
    case(
        "5 > 2".to_string(),
        "test_evaluator_infix_boolean_expressions_case2"
    ),
    case(
        "3 > 3".to_string(),
        "test_evaluator_infix_boolean_expressions_case3"
    ),
    case(
        "7 < 7".to_string(),
        "test_evaluator_infix_boolean_expressions_case4"
    ),
    case(
        "13 == 13 ".to_string(),
        "test_evaluator_infix_boolean_expressions_case5"
    ),
    case(
        "11 == 19 ".to_string(),
        "test_evaluator_infix_boolean_expressions_case6"
    ),
    case(
        "9 != 15".to_string(),
        "test_evaluator_infix_boolean_expressions_case7"
    ),
    case(
        "7 != 7".to_string(),
        "test_evaluator_infix_boolean_expressions_case8"
    ),
    case(
        "true == true".to_string(),
        "test_evaluator_infix_boolean_expressions_case9"
    ),
    case(
        "false == false".to_string(),
        "test_evaluator_infix_boolean_expressions_case10"
    ),
    case(
        "true == false".to_string(),
        "test_evaluator_infix_boolean_expressions_case11"
    ),
    case(
        "false == true".to_string(),
        "test_evaluator_infix_boolean_expressions_case12"
    ),
    case(
        "true != false".to_string(),
        "test_evaluator_infix_boolean_expressions_case13"
    ),
    case(
        "false != true".to_string(),
        "test_evaluator_infix_boolean_expressions_case14"
    ),
    case(
        "true != true".to_string(),
        "test_evaluator_infix_boolean_expressions_case15"
    ),
    case(
        "false != false".to_string(),
        "test_evaluator_infix_boolean_expressions_case16"
    ),
    case(
        "(1 < 2) == true".to_string(),
        "test_evaluator_infix_boolean_expressions_case17"
    ),
    case(
        "(1 < 2) == false".to_string(),
        "test_evaluator_infix_boolean_expressions_case18"
    ),
    case(
        "(1 > 2) == true".to_string(),
        "test_evaluator_infix_boolean_expressions_case19"
    ),
    case(
        "(1 > 2) == false".to_string(),
        "test_evaluator_infix_boolean_expressions_case20"
    ),
)]
fn test_evaluator_infix_boolean_expressions(code: String, snapshot_name: &str) {
    assert_expected_returned_object!(code, snapshot_name);
}