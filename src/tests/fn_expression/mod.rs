use super::*;

#[template]
#[rstest(
    code,
    snapshot_name,
    case("fn(x) {\n\tx + 3\n}\n", "fn_expression_case_1"),
    case("fn(x,y) { x + y; };", "fn_expression_case_2"),
    case("FN() {\n3\n}\n", "fn_expression_case_3"),
    case("fn(x,y) {\n\treturn x + y;\n}", "fn_expression_case_4"),
    case("fn(){}", "fn_expression_case_5")
)]
fn fn_expression_cases(code: &str, snapshot_name: &str) {}

#[apply(fn_expression_cases)]
fn test_fn_expression_lexical_analysis(code: &str, snapshot_name: &str) {
    assert_lexical_analysis!(code, snapshot_name);
}

#[apply(fn_expression_cases)]
fn test_fn_expression_syntax_analysis(code: &str, snapshot_name: &str) {
    assert_syntax_analysis!(code, snapshot_name);
}

#[apply(fn_expression_cases)]
fn test_fn_expression_evaluation(code: &str, snapshot_name: &str) {
    assert_evaluation!(code, snapshot_name);
}

#[apply(fn_expression_cases)]
fn test_fn_expression_environment(code: &str, snapshot_name: &str) {
    assert_environment!(code, snapshot_name);
}
