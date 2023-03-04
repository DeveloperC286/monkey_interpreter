use super::*;

#[template]
#[rstest(
    code,
    snapshot_name,
    case("51a23", "invalid_integer_expression_case_1"),
    case("  123.0", "invalid_integer_expression_case_2"),
    case("\t0xFF", "invalid_integer_expression_case_3")
)]
fn invalid_integer_expression_cases(code: &str, snapshot_name: &str) {}

#[apply(invalid_integer_expression_cases)]
fn test_invalid_integer_expression_lexical_analysis_error(code: &str, snapshot_name: &str) {
    assert_lexical_analysis_error!(code, snapshot_name);
}
