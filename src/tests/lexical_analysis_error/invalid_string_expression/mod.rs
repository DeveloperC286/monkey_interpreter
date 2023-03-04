use super::*;

#[template]
#[rstest(
    code,
    snapshot_name,
    case("\"this is a invalid_string", "invalid_string_expression_case_1"),
    case("'this is the wrong quotes'", "invalid_string_expression_case_2"),
    case(
        "let string = \"this is a invalid_string;",
        "invalid_string_expression_case_3"
    )
)]
fn invalid_string_expression_cases(code: &str, snapshot_name: &str) {}

#[apply(invalid_string_expression_cases)]
fn test_invalid_string_expression_lexical_analysis_error(code: &str, snapshot_name: &str) {
    assert_lexical_analysis_error!(code, snapshot_name);
}
