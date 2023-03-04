use super::*;

#[template]
#[rstest(
    code,
    snapshot_name,
    case("let sentence = -;", "invalid_integer_prefix_expression_case_1"),
    case("-", "invalid_integer_prefix_expression_case_2"),
    case("\tlet file = -;\n", "invalid_integer_prefix_expression_case_3")
)]
fn invalid_integer_prefix_expression_cases(code: &str, snapshot_name: &str) {}

#[apply(invalid_integer_prefix_expression_cases)]
fn test_invalid_integer_prefix_expression_lexical_analysis(code: &str, snapshot_name: &str) {
    assert_lexical_analysis!(code, snapshot_name);
}

#[apply(invalid_integer_prefix_expression_cases)]
fn test_invalid_integer_prefix_expression_syntax_analysis_error(code: &str, snapshot_name: &str) {
    assert_syntax_analysis_error!(code, snapshot_name);
}
