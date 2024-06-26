use super::*;

#[template]
#[rstest(
    code,
    snapshot_name,
    case("(1 > 2) == 5", "type_mismatch_boolean_infix_expression_case_1"),
    case(
        "\"this is a string\" == TRUE",
        "type_mismatch_boolean_infix_expression_case_2"
    ),
    case(
        "82 != \"this is a string\"",
        "type_mismatch_boolean_infix_expression_case_3"
    )
)]
fn type_mismatch_boolean_infix_expression_cases(code: &str, snapshot_name: &str) {}

#[apply(type_mismatch_boolean_infix_expression_cases)]
fn test_type_mismatch_boolean_infix_expression_lexical_analysis(code: &str, snapshot_name: &str) {
    assert_lexical_analysis!(code, snapshot_name);
}

#[apply(type_mismatch_boolean_infix_expression_cases)]
fn test_type_mismatch_boolean_infix_expression_syntax_analysis(code: &str, snapshot_name: &str) {
    assert_syntax_analysis!(code, snapshot_name);
}

#[apply(type_mismatch_boolean_infix_expression_cases)]
fn test_type_mismatch_boolean_infix_expression_evaluation_error(code: &str, snapshot_name: &str) {
    assert_evaluation_error!(code, snapshot_name);
}
