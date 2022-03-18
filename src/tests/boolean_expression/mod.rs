use super::*;

#[template]
#[rstest(
    code,
    snapshot_name,
    case("false", "boolean_expression_case_1"),
    case("FALSE", "boolean_expression_case_2"),
    case("true", "boolean_expression_case_3"),
    case("TRUE", "boolean_expression_case_4")
)]
fn boolean_expression_cases(code: &str, snapshot_name: &str) {}

#[apply(boolean_expression_cases)]
fn test_boolean_expression_lexical_analysis(code: &str, snapshot_name: &str) {
    assert_lexical_analysis!(code, snapshot_name);
}

#[apply(boolean_expression_cases)]
fn test_boolean_expression_syntax_analysis(code: &str, snapshot_name: &str) {
    assert_syntax_analysis!(code, snapshot_name);
}

#[apply(boolean_expression_cases)]
fn test_boolean_expression_evaluator(code: &str, snapshot_name: &str) {
    assert_evaluator!(code, snapshot_name);
}

#[apply(boolean_expression_cases)]
fn test_boolean_expression_environment(code: &str, snapshot_name: &str) {
    assert_environment!(code, snapshot_name);
}
