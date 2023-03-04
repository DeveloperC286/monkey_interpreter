use super::*;

#[template]
#[rstest(
    code,
    snapshot_name,
    case("fn(x) { x; }(5);", "call_expression_case_1"),
    case("fn(x) { return x; }(5);", "call_expression_case_2"),
    case("fn(x) { x * 2 }(5);", "call_expression_case_3"),
    case("fn(x, y) { x + y }(2, 3);", "call_expression_case_4"),
    case(
        "let add = fn(x, y) { x + y }; add(1, add(2, 3));",
        "call_expression_case_5"
    )
)]
fn call_expression_cases(code: &str, snapshot_name: &str) {}

#[apply(call_expression_cases)]
fn test_call_expression_lexical_analysis(code: &str, snapshot_name: &str) {
    assert_lexical_analysis!(code, snapshot_name);
}

#[apply(call_expression_cases)]
fn test_call_expression_syntax_analysis(code: &str, snapshot_name: &str) {
    assert_syntax_analysis!(code, snapshot_name);
}

#[apply(call_expression_cases)]
fn test_call_expression_evaluation(code: &str, snapshot_name: &str) {
    assert_evaluation!(code, snapshot_name);
}

#[apply(call_expression_cases)]
fn test_call_expression_environment(code: &str, snapshot_name: &str) {
    assert_environment!(code, snapshot_name);
}
