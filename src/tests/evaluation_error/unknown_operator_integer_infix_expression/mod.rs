use super::*;

#[template]
#[rstest(
    code,
    snapshot_name,
    case("TRUE - TRUE", "unknown_operator_integer_infix_expression_case_2"),
    case("TRUE * TRUE", "unknown_operator_integer_infix_expression_case_3"),
    case("false / false", "unknown_operator_integer_infix_expression_case_4")
)]
fn unknown_operator_integer_infix_expression_cases(code: &str, snapshot_name: &str) {}

#[apply(unknown_operator_integer_infix_expression_cases)]
fn test_unknown_operator_integer_infix_expression_lexical_analysis(
    code: &str,
    snapshot_name: &str,
) {
    assert_lexical_analysis!(code, snapshot_name);
}

#[apply(unknown_operator_integer_infix_expression_cases)]
fn test_unknown_operator_integer_infix_expression_syntax_analysis(code: &str, snapshot_name: &str) {
    assert_syntax_analysis!(code, snapshot_name);
}

#[apply(unknown_operator_integer_infix_expression_cases)]
fn test_unknown_operator_integer_infix_expression_evaluation_error(
    code: &str,
    snapshot_name: &str,
) {
    assert_evaluation_error!(code, snapshot_name);
}
