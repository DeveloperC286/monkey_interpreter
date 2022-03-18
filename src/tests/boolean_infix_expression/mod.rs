use super::*;

#[template]
#[rstest(
    code,
    snapshot_name,
    case("true == true", "boolean_infix_expression_case_1"),
    case("false == false", "boolean_infix_expression_case_2"),
    case("true == false", "boolean_infix_expression_case_3"),
    case("false == true", "boolean_infix_expression_case_4"),
    case("true != false", "boolean_infix_expression_case_5"),
    case("false != true", "boolean_infix_expression_case_6"),
    case("true != true", "boolean_infix_expression_case_7"),
    case("false != false", "boolean_infix_expression_case_8")
)]
fn boolean_infix_expression_cases(code: &str, snapshot_name: &str) {}

#[apply(boolean_infix_expression_cases)]
fn test_boolean_infix_expression_lexical_analysis(code: &str, snapshot_name: &str) {
    assert_lexical_analysis!(code, snapshot_name);
}

#[apply(boolean_infix_expression_cases)]
fn test_boolean_infix_expression_syntax_analysis(code: &str, snapshot_name: &str) {
    assert_syntax_analysis!(code, snapshot_name);
}

#[apply(boolean_infix_expression_cases)]
fn test_boolean_infix_expression_evaluator(code: &str, snapshot_name: &str) {
    assert_evaluator!(code, snapshot_name);
}

#[apply(boolean_infix_expression_cases)]
fn test_boolean_infix_expression_environment(code: &str, snapshot_name: &str) {
    assert_environment!(code, snapshot_name);
}
