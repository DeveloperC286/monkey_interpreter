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

#[rstest(
    code,
    snapshot_name,
    case(
        "let string = \"this is a string",
        "test_lexical_analysis_tokenization_for_string_lexical_errors_case1"
    ),
    case(
        "let sentence = \"this is a string\\x\";",
        "test_lexical_analysis_tokenization_for_string_lexical_errors_case2"
    ),
    case(
        "\"this string is not quoted\\",
        "test_lexical_analysis_tokenization_for_string_lexical_errors_case3"
    )
)]
fn test_lexical_analysis_tokenization_for_string_lexical_errors(code: &str, snapshot_name: &str) {
    assert_expected_returned_tokens!(code, snapshot_name);
}
