use super::*;

#[template]
#[rstest(
    code,
    snapshot_name,
    case("FALSE + TRUE", "type_mismatch_plus_infix_expression_case_1"),
    case("5 + TRUE", "type_mismatch_plus_infix_expression_case_2"),
    case("\"123\" + false", "type_mismatch_plus_infix_expression_case_3"),
    case("10 + \" cant append\"", "type_mismatch_plus_infix_expression_case_4"),
    case("\"456\" + 7", "type_mismatch_plus_infix_expression_case_5")
)]
fn type_mismatch_plus_infix_expression_cases(code: &str, snapshot_name: &str) {}

#[apply(type_mismatch_plus_infix_expression_cases)]
fn test_type_mismatch_plus_infix_expression_lexical_analysis(code: &str, snapshot_name: &str) {
    assert_lexical_analysis!(code, snapshot_name);
}

#[apply(type_mismatch_plus_infix_expression_cases)]
fn test_type_mismatch_plus_infix_expression_syntax_analysis(code: &str, snapshot_name: &str) {
    assert_syntax_analysis!(code, snapshot_name);
}

#[apply(type_mismatch_plus_infix_expression_cases)]
fn test_type_mismatch_plus_infix_expression_evaluation_error(code: &str, snapshot_name: &str) {
    assert_evaluation_error!(code, snapshot_name);
}
