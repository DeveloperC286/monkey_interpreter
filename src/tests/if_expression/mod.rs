use super::*;

#[template]
#[rstest(
    code,
    snapshot_name,
    case(
        "if (10 > 1) { if (10 > 1) { return TRUE;} return FALSE;}",
        "if_expression_case_1"
    ),
    case("if (10 > 1) { return 1 + 5; } return 1;", "if_expression_case_2"),
    case("if (1) { 1 }", "if_expression_case_3"),
    case("if (1 < 2) { 4 } else { true }", "if_expression_case_4"),
    case("if (true) { 10 }", "if_expression_case_5"),
    case("if (false) { 10 }", "if_expression_case_6"),
    case("if (1 > 2) { 5 }", "if_expression_case_7"),
    case("if (4 > 6) { true } else { false }", "if_expression_case_8"),
    case("if (false) { 85 } else { 58 }", "if_expression_case_9")
)]
fn if_expression_cases(code: &str, snapshot_name: &str) {}

#[apply(if_expression_cases)]
fn test_if_expression_lexical_analysis(code: &str, snapshot_name: &str) {
    assert_lexical_analysis!(code, snapshot_name);
}

#[apply(if_expression_cases)]
fn test_if_expression_syntax_analysis(code: &str, snapshot_name: &str) {
    assert_syntax_analysis!(code, snapshot_name);
}

#[apply(if_expression_cases)]
fn test_if_expression_evaluation(code: &str, snapshot_name: &str) {
    assert_evaluation!(code, snapshot_name);
}

#[apply(if_expression_cases)]
fn test_if_expression_environment(code: &str, snapshot_name: &str) {
    assert_environment!(code, snapshot_name);
}
