use super::*;

#[template]
#[rstest(code, snapshot_name, case("", "edge_cases_case_1"))]
fn edge_cases_cases(code: &str, snapshot_name: &str) {}

#[apply(edge_cases_cases)]
fn test_edge_cases_lexical_analysis(code: &str, snapshot_name: &str) {
    assert_lexical_analysis!(code, snapshot_name);
}

#[apply(edge_cases_cases)]
fn test_edge_cases_syntax_analysis(code: &str, snapshot_name: &str) {
    assert_syntax_analysis!(code, snapshot_name);
}

#[apply(edge_cases_cases)]
fn test_edge_cases_evaluator(code: &str, snapshot_name: &str) {
    assert_evaluator!(code, snapshot_name);
}

#[apply(edge_cases_cases)]
fn test_edge_cases_environment(code: &str, snapshot_name: &str) {
    assert_environment!(code, snapshot_name);
}
