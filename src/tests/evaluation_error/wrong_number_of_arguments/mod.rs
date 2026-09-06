use super::*;

#[template]
#[rstest(
    code,
    snapshot_name,
    case("fn(x, y) { x + y }(1);", "wrong_number_of_arguments_case_1"),
    case(
        "let add = fn(x, y) { x + y }; add(1, 2, 3);",
        "wrong_number_of_arguments_case_2"
    ),
    case("fn(x) { x }();", "wrong_number_of_arguments_case_3"),
    case("fn() { 5 }(1);", "wrong_number_of_arguments_case_4")
)]
fn wrong_number_of_arguments_cases(code: &str, snapshot_name: &str) {}

#[apply(wrong_number_of_arguments_cases)]
fn test_wrong_number_of_arguments_lexical_analysis(code: &str, snapshot_name: &str) {
    assert_lexical_analysis!(code, snapshot_name);
}

#[apply(wrong_number_of_arguments_cases)]
fn test_wrong_number_of_arguments_syntax_analysis(code: &str, snapshot_name: &str) {
    assert_syntax_analysis!(code, snapshot_name);
}

#[apply(wrong_number_of_arguments_cases)]
fn test_wrong_number_of_arguments_evaluation_error(code: &str, snapshot_name: &str) {
    assert_evaluation_error!(code, snapshot_name);
}
