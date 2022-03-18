use super::*;

#[template]
#[rstest(
    code,
    snapshot_name,
    case("\t-5", "integer_prefix_expression_case_1"),
    case("  -10", "integer_prefix_expression_case_2")
)]
fn integer_prefix_expression_cases(code: &str, snapshot_name: &str) {}

#[apply(integer_prefix_expression_cases)]
fn test_integer_prefix_expression_lexical_analysis(code: &str, snapshot_name: &str) {
    assert_lexical_analysis!(code, snapshot_name);
}

#[apply(integer_prefix_expression_cases)]
fn test_integer_prefix_expression_syntax_analysis(code: &str, snapshot_name: &str) {
    assert_syntax_analysis!(code, snapshot_name);
}

#[apply(integer_prefix_expression_cases)]
fn test_integer_prefix_expression_evaluator(code: &str, snapshot_name: &str) {
    assert_evaluator!(code, snapshot_name);
}

#[apply(integer_prefix_expression_cases)]
fn test_integer_prefix_expression_environment(code: &str, snapshot_name: &str) {
    assert_environment!(code, snapshot_name);
}
