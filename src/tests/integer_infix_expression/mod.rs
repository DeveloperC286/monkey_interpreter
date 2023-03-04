use super::*;

#[template]
#[rstest(
    code,
    snapshot_name,
    case("\t-(5 * 10)", "integer_infix_expression_case_1"),
    case("(5 - 10) * 10;", "integer_infix_expression_case_2"),
    case("15 + 5", "integer_infix_expression_case_3"),
    case("6 - 6", "integer_infix_expression_case_4"),
    case("3 * 4", "integer_infix_expression_case_5"),
    case("5 / 10", "integer_infix_expression_case_6"),
    case("-5 * 10", "integer_infix_expression_case_7"),
    case("5 - 10 * 10", "integer_infix_expression_case_8"),
    case("10 / 5 + 1", "integer_infix_expression_case_9"),
    case("-3 - -3", "integer_infix_expression_case_10"),
    case("37 * -1", "integer_infix_expression_case_11"),
    case("5 + 5  + 10", "integer_infix_expression_case_12"),
    case("-100 / 10", "integer_infix_expression_case_13")
)]
fn integer_infix_expression_cases(code: &str, snapshot_name: &str) {}

#[apply(integer_infix_expression_cases)]
fn test_integer_infix_expression_lexical_analysis(code: &str, snapshot_name: &str) {
    assert_lexical_analysis!(code, snapshot_name);
}

#[apply(integer_infix_expression_cases)]
fn test_integer_infix_expression_syntax_analysis(code: &str, snapshot_name: &str) {
    assert_syntax_analysis!(code, snapshot_name);
}

#[apply(integer_infix_expression_cases)]
fn test_integer_infix_expression_evaluation(code: &str, snapshot_name: &str) {
    assert_evaluation!(code, snapshot_name);
}

#[apply(integer_infix_expression_cases)]
fn test_integer_infix_expression_environment(code: &str, snapshot_name: &str) {
    assert_environment!(code, snapshot_name);
}
