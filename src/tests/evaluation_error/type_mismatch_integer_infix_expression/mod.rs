use super::*;

#[template]
#[rstest(
    code,
    snapshot_name,
    case("10 / false", "type_mismatch_integer_infix_expression_case_4"),
    case(
        "\"divide me \" / TRUE",
        "type_mismatch_integer_infix_expression_case_8"
    )
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
fn test_type_mismatch_integer_infix_expression_evaluation_error(code: &str, snapshot_name: &str) {
    assert_evaluation_error!(code, snapshot_name);
}
