use super::*;

#[template]
#[rstest(
    code,
    snapshot_name,
    case("5", "integer_expression_case_1"),
    case("  123", "integer_expression_case_2")
)]
fn integer_expression_cases(code: &str, snapshot_name: &str) {}

#[apply(integer_expression_cases)]
fn test_integer_expression_lexical_analysis(code: &str, snapshot_name: &str) {
    assert_lexical_analysis!(code, snapshot_name);
}

#[apply(integer_expression_cases)]
fn test_integer_expression_syntax_analysis(code: &str, snapshot_name: &str) {
    assert_syntax_analysis!(code, snapshot_name);
}

#[apply(integer_expression_cases)]
fn test_integer_expression_evaluation(code: &str, snapshot_name: &str) {
    assert_evaluation!(code, snapshot_name);
}

#[apply(integer_expression_cases)]
fn test_integer_expression_environment(code: &str, snapshot_name: &str) {
    assert_environment!(code, snapshot_name);
}
