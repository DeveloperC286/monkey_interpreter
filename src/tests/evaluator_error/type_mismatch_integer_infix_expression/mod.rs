use super::*;

#[template]
#[rstest(
    code,
    snapshot_name,
    case("5 + TRUE", "type_mismatch_integer_infix_expression_case_1"),
    case("10 - TRUE", "type_mismatch_integer_infix_expression_case_2"),
    case("TRUE * 9", "type_mismatch_integer_infix_expression_case_3"),
    case("10 / false", "type_mismatch_integer_infix_expression_case_4"),
    case("\"123\" + false", "type_mismatch_integer_infix_expression_case_5"),
    case(
        "true - \"this is a string\"",
        "type_mismatch_integer_infix_expression_case_6"
    ),
    case(
        "FALSE * \"can't multiply\"",
        "type_mismatch_integer_infix_expression_case_7"
    ),
    case(
        "\"divide me \" / TRUE",
        "type_mismatch_integer_infix_expression_case_8"
    ),
    case(
        "10 + \" cant append\"",
        "type_mismatch_integer_infix_expression_case_9"
    ),
    case("\" 123\" - 10 ", "type_mismatch_integer_infix_expression_case_10"),
    case("\"456\" + 7", "type_mismatch_integer_infix_expression_case_11"),
    case("\"456\" - 7", "type_mismatch_integer_infix_expression_case_12"),
    case("\"456\" * 4", "type_mismatch_integer_infix_expression_case_13"),
    case("12 * \"muliply\"", "type_mismatch_integer_infix_expression_case_14")
)]
fn type_mismatch_integer_infix_expression_cases(code: &str, snapshot_name: &str) {}

#[apply(type_mismatch_integer_infix_expression_cases)]
fn test_type_mismatch_integer_infix_expression_lexical_analysis(code: &str, snapshot_name: &str) {
    assert_lexical_analysis!(code, snapshot_name);
}

#[apply(type_mismatch_integer_infix_expression_cases)]
fn test_type_mismatch_integer_infix_expression_syntax_analysis(code: &str, snapshot_name: &str) {
    assert_syntax_analysis!(code, snapshot_name);
}

#[apply(type_mismatch_integer_infix_expression_cases)]
fn test_type_mismatch_integer_infix_expression_evaluator(code: &str, snapshot_name: &str) {
    assert_evaluator_error!(code, snapshot_name);
}
