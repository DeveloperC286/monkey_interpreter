use super::*;

#[template]
#[rstest(
    code,
    snapshot_name,
    case("return 10;", "return_statement_case_1"),
    case("return true;", "return_statement_case_2"),
    case("return \"string123\";", "return_statement_case_3")
)]
fn return_statement_cases(code: &str, snapshot_name: &str) {}

#[apply(return_statement_cases)]
fn test_return_statement_lexical_analysis(code: &str, snapshot_name: &str) {
    assert_lexical_analysis!(code, snapshot_name);
}

#[apply(return_statement_cases)]
fn test_return_statement_syntax_analysis(code: &str, snapshot_name: &str) {
    assert_syntax_analysis!(code, snapshot_name);
}

#[apply(return_statement_cases)]
fn test_return_statement_evaluation(code: &str, snapshot_name: &str) {
    assert_evaluation!(code, snapshot_name);
}

#[apply(return_statement_cases)]
fn test_return_statement_environment(code: &str, snapshot_name: &str) {
    assert_environment!(code, snapshot_name);
}
