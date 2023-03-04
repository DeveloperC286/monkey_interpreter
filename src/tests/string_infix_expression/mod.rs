use super::*;

#[template]
#[rstest(
    code,
    snapshot_name,
    case("\"123\" + \"456\"", "string_infix_expression_case_1")
)]
fn string_infix_expression_cases(code: &str, snapshot_name: &str) {}

#[apply(string_infix_expression_cases)]
fn test_string_infix_expression_lexical_analysis(code: &str, snapshot_name: &str) {
    assert_lexical_analysis!(code, snapshot_name);
}

#[apply(string_infix_expression_cases)]
fn test_string_infix_expression_syntax_analysis(code: &str, snapshot_name: &str) {
    assert_syntax_analysis!(code, snapshot_name);
}

#[apply(string_infix_expression_cases)]
fn test_string_infix_expression_evaluation(code: &str, snapshot_name: &str) {
    assert_evaluation!(code, snapshot_name);
}

#[apply(string_infix_expression_cases)]
fn test_string_infix_expression_environment(code: &str, snapshot_name: &str) {
    assert_environment!(code, snapshot_name);
}
