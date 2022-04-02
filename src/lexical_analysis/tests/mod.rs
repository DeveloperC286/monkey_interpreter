use rstest::rstest;

#[macro_use]
mod macros;

#[rstest(
    code,
    snapshot_name,
    case("let x = `2`;", "test_lexical_analysis_producing_lexical_errors_case1"),
    case(
        "let x = 2;\n let z = &x;",
        "test_lexical_analysis_producing_lexical_errors_case2"
    ),
    case(
        "let x =| sum(2);",
        "test_lexical_analysis_producing_lexical_errors_case3"
    )
)]
fn test_lexical_analysis_producing_lexical_errors(code: &str, snapshot_name: &str) {
    assert_expected_returned_tokens!(code, snapshot_name);
}
