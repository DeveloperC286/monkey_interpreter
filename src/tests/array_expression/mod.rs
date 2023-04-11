use super::*;

#[template]
#[rstest(code, snapshot_name, case("[1, 2, 3]", "array_expression_case_1"))]
fn array_expression_cases(code: &str, snapshot_name: &str) {}

#[apply(array_expression_cases)]
fn test_array_expression_lexical_analysis(code: &str, snapshot_name: &str) {
    assert_lexical_analysis!(code, snapshot_name);
}
