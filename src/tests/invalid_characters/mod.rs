use super::*;

#[template]
#[rstest(
    code,
    snapshot_name,
    case("let x = `2`;", "invalid_characters_case_1"),
    case("let x = 2;\n let z = &x;", "invalid_characters_case_2"),
    case("let x =| sum(2);", "invalid_characters_case_3")
)]
fn invalid_characters_cases(code: &str, snapshot_name: &str) {}

#[apply(invalid_characters_cases)]
fn test_invalid_characters_lexical_analysis(code: &str, snapshot_name: &str) {
    assert_lexical_analysis_error!(code, snapshot_name);
}
