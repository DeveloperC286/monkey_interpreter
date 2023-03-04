use super::*;

#[template]
#[rstest(
    code,
    snapshot_name,
    case("\"this is a string\"", "string_expression_case_1"),
    case("\"line1\nline2\n\"", "string_expression_case_2"),
    case("\"column-1\\tcolumn-2\\n\"", "string_expression_case_3")
)]
fn string_expression_cases(code: &str, snapshot_name: &str) {}

#[apply(string_expression_cases)]
fn test_string_expression_lexical_analysis(code: &str, snapshot_name: &str) {
    assert_lexical_analysis!(code, snapshot_name);
}

#[apply(string_expression_cases)]
fn test_string_expression_syntax_analysis(code: &str, snapshot_name: &str) {
    assert_syntax_analysis!(code, snapshot_name);
}

#[apply(string_expression_cases)]
fn test_string_expression_evaluation(code: &str, snapshot_name: &str) {
    assert_evaluation!(code, snapshot_name);
}

#[apply(string_expression_cases)]
fn test_string_expression_environment(code: &str, snapshot_name: &str) {
    assert_environment!(code, snapshot_name);
}
