use super::*;

#[template]
#[rstest(
    code,
    snapshot_name,
    case("!5", "type_mismatch_integer_prefix_expression_case_1")
)]
fn type_mismatch_integer_prefix_expression_cases(code: &str, snapshot_name: &str) {}

#[apply(type_mismatch_integer_prefix_expression_cases)]
fn test_type_mismatch_integer_prefix_expression_lexical_analysis(code: &str, snapshot_name: &str) {
    assert_lexical_analysis!(code, snapshot_name);
}

#[apply(type_mismatch_integer_prefix_expression_cases)]
fn test_type_mismatch_integer_prefix_expression_syntax_analysis(code: &str, snapshot_name: &str) {
    assert_syntax_analysis!(code, snapshot_name);
}

#[apply(type_mismatch_integer_prefix_expression_cases)]
fn test_type_mismatch_integer_prefix_expression_evaluation_error(code: &str, snapshot_name: &str) {
    assert_evaluation_error!(code, snapshot_name);
}
