use super::*;

#[template]
#[rstest(
    code,
    snapshot_name,
    case("TRUE * TRUE", "type_mismatch_multiply_infix_expression_case_1"),
    case("TRUE * 9", "type_mismatch_multiply_infix_expression_case_2"),
    case(
        "FALSE * \"can't multiply\"",
        "type_mismatch_multiply_infix_expression_case_3"
    ),
    case("\"456\" * 4", "type_mismatch_multiply_infix_expression_case_4"),
    case("12 * \"muliply\"", "type_mismatch_multiply_infix_expression_case_5")
)]
fn type_mismatch_multiply_infix_expression_cases(code: &str, snapshot_name: &str) {}

#[apply(type_mismatch_multiply_infix_expression_cases)]
fn test_type_mismatch_multiply_infix_expression_lexical_analysis(code: &str, snapshot_name: &str) {
    assert_lexical_analysis!(code, snapshot_name);
}

#[apply(type_mismatch_multiply_infix_expression_cases)]
fn test_type_mismatch_multiply_infix_expression_syntax_analysis(code: &str, snapshot_name: &str) {
    assert_syntax_analysis!(code, snapshot_name);
}

#[apply(type_mismatch_multiply_infix_expression_cases)]
fn test_type_mismatch_multiply_infix_expression_evaluation_error(code: &str, snapshot_name: &str) {
    assert_evaluation_error!(code, snapshot_name);
}
