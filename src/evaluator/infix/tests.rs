use insta::assert_json_snapshot;
use rstest::rstest;

#[rstest(
    code,
    snapshot_name,
    case(
        "5 + 5".to_string(),
        "test_evaluator_infix_integer_nodes_case1"
    ),
    case(
        "5 + 5  + 10".to_string(),
        "test_evaluator_infix_integer_nodes_case2"
    ),
    case(
        "10 - 15".to_string(),
        "test_evaluator_infix_integer_nodes_case3"
    ),
    case(
        "-3 - -3".to_string(),
        "test_evaluator_infix_integer_nodes_case4"
    ),
    case(
        "2 * 12".to_string(),
        "test_evaluator_infix_integer_nodes_case5"
    ),
    case(
        "37 * -1".to_string(),
        "test_evaluator_infix_integer_nodes_case6"
    ),
    case(
        "20 / 10".to_string(),
        "test_evaluator_infix_integer_nodes_case7"
    ),
    case(
        "-100 / 10".to_string(),
        "test_evaluator_infix_integer_nodes_case8"
    ),
    case(
        "5 + 5 * 10".to_string(),
        "test_evaluator_infix_integer_nodes_case9"
    ),
    case(
        "(5 + 5) / 10".to_string(),
        "test_evaluator_infix_integer_nodes_case10"
    ),
)]
fn test_evaluator_infix_integer_nodes(code: String, snapshot_name: &str) {
    assert_expected_returned_object!(code, snapshot_name);
}
