use super::*;

#[template]
#[rstest(
    code,
    snapshot_name,
    case("2 < 3", "integer_infix_boolean_expression_case_1"),
    case("5 > 2", "integer_infix_boolean_expression_case_2"),
    case("3 > 3", "integer_infix_boolean_expression_case_3"),
    case("7 < 7", "integer_infix_boolean_expression_case_4"),
    case("13 == 13", "integer_infix_boolean_expression_case_5"),
    case("11 == 19", "integer_infix_boolean_expression_case_6"),
    case("9 != 15", "integer_infix_boolean_expression_case_7"),
    case("7 != 7", "integer_infix_boolean_expression_case_8"),
    case("1 < 2 == true", "integer_infix_boolean_expression_case_9"),
    case("1 < 2 == false", "integer_infix_boolean_expression_case_10"),
    case("1 > 2 == true", "integer_infix_boolean_expression_case_11"),
    case("1 > 2 == false", "integer_infix_boolean_expression_case_12")
)]
fn integer_infix_boolean_expression_cases(code: &str, snapshot_name: &str) {}

#[apply(integer_infix_boolean_expression_cases)]
fn test_integer_infix_boolean_expression_lexical_analysis(code: &str, snapshot_name: &str) {
    assert_lexical_analysis!(code, snapshot_name);
}

#[apply(integer_infix_boolean_expression_cases)]
fn test_integer_infix_boolean_expression_syntax_analysis(code: &str, snapshot_name: &str) {
    assert_syntax_analysis!(code, snapshot_name);
}

#[apply(integer_infix_boolean_expression_cases)]
fn test_integer_infix_boolean_expression_evaluator(code: &str, snapshot_name: &str) {
    assert_evaluator!(code, snapshot_name);
}

#[apply(integer_infix_boolean_expression_cases)]
fn test_integer_infix_boolean_expression_environment(code: &str, snapshot_name: &str) {
    assert_environment!(code, snapshot_name);
}
