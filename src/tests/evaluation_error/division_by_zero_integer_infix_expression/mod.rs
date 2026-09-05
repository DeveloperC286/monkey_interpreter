use super::*;

#[template]
#[rstest(
    code,
    snapshot_name,
    case("5 / 0", "division_by_zero_integer_infix_expression_case_1"),
    case("0 / 0", "division_by_zero_integer_infix_expression_case_2"),
    case("-5 / 0", "division_by_zero_integer_infix_expression_case_3"),
    case("10 / (5 - 5)", "division_by_zero_integer_infix_expression_case_4"),
    case(
        "let denominator = 0; 100 / denominator;",
        "division_by_zero_integer_infix_expression_case_5"
    )
)]
fn division_by_zero_integer_infix_expression_cases(code: &str, snapshot_name: &str) {}

#[apply(division_by_zero_integer_infix_expression_cases)]
fn test_division_by_zero_integer_infix_expression_lexical_analysis(
    code: &str,
    snapshot_name: &str,
) {
    assert_lexical_analysis!(code, snapshot_name);
}

#[apply(division_by_zero_integer_infix_expression_cases)]
fn test_division_by_zero_integer_infix_expression_syntax_analysis(code: &str, snapshot_name: &str) {
    assert_syntax_analysis!(code, snapshot_name);
}

#[apply(division_by_zero_integer_infix_expression_cases)]
fn test_division_by_zero_integer_infix_expression_evaluation_error(
    code: &str,
    snapshot_name: &str,
) {
    assert_evaluation_error!(code, snapshot_name);
}
