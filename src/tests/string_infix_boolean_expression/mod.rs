use super::*;

#[template]
#[rstest(
    code,
    snapshot_name,
    case("\"name\" == \"name\"", "string_infix_boolean_expression_case_1"),
    case(
        "\"this is a string\" == \"this is a string\"",
        "string_infix_boolean_expression_case_2"
    ),
    case("\"first\" != \"last\"", "string_infix_boolean_expression_case_3"),
    case(
        "\"string123\" != \"\\tstring123\"",
        "string_infix_boolean_expression_case_4"
    ),
    case(
        "\"string123\" != \"string123\"",
        "string_infix_boolean_expression_case_5"
    ),
    case("\"name\" == \"name123\"", "string_infix_boolean_expression_case_6")
)]
fn string_infix_boolean_expression_cases(code: &str, snapshot_name: &str) {}

#[apply(string_infix_boolean_expression_cases)]
fn test_string_infix_boolean_expression_lexical_analysis(code: &str, snapshot_name: &str) {
    assert_lexical_analysis!(code, snapshot_name);
}

#[apply(string_infix_boolean_expression_cases)]
fn test_string_infix_boolean_expression_syntax_analysis(code: &str, snapshot_name: &str) {
    assert_syntax_analysis!(code, snapshot_name);
}

#[apply(string_infix_boolean_expression_cases)]
fn test_string_infix_boolean_expression_evaluator(code: &str, snapshot_name: &str) {
    assert_evaluator!(code, snapshot_name);
}

#[apply(string_infix_boolean_expression_cases)]
fn test_string_infix_boolean_expression_environment(code: &str, snapshot_name: &str) {
    assert_environment!(code, snapshot_name);
}
