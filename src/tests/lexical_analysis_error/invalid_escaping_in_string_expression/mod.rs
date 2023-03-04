use super::*;

#[template]
#[rstest(
    code,
    snapshot_name,
    case(
        "let sentence = \"this is a string\\x\";",
        "invalid_escaping_in_string_expression_case_1"
    )
)]
fn invalid_escaping_in_string_expression_cases(code: &str, snapshot_name: &str) {}

#[apply(invalid_escaping_in_string_expression_cases)]
fn test_invalid_escaping_in_string_expression_lexical_analysis_error(
    code: &str,
    snapshot_name: &str,
) {
    assert_lexical_analysis_error!(code, snapshot_name);
}
