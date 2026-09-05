use super::*;

#[template]
#[rstest(
    code,
    snapshot_name,
    case("5(1);", "uncallable_object_case_1"),
    case("\"this is a string\"();", "uncallable_object_case_2"),
    case("true(1);", "uncallable_object_case_3"),
    case("(1 + 2)(3);", "uncallable_object_case_4"),
    case("let five = 5; five(1);", "uncallable_object_case_5"),
    case("fn(x) { x }(1)(2);", "uncallable_object_case_6"),
    case("if (true) { 1 } else { 2 }(3);", "uncallable_object_case_7")
)]
fn uncallable_object_cases(code: &str, snapshot_name: &str) {}

#[apply(uncallable_object_cases)]
fn test_uncallable_object_lexical_analysis(code: &str, snapshot_name: &str) {
    assert_lexical_analysis!(code, snapshot_name);
}

#[apply(uncallable_object_cases)]
fn test_uncallable_object_syntax_analysis(code: &str, snapshot_name: &str) {
    assert_syntax_analysis!(code, snapshot_name);
}

#[apply(uncallable_object_cases)]
fn test_uncallable_object_evaluation_error(code: &str, snapshot_name: &str) {
    assert_evaluation_error!(code, snapshot_name);
}
