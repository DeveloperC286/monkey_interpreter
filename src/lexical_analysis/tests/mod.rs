use rstest::rstest;

#[macro_use]
mod macros;

#[rstest(
    code,
    snapshot_name,
    case("\t123 \n", "test_lexical_analysis_tokenization_for_integers_case1")
)]
fn test_lexical_analysis_tokenization_for_integers(code: &str, snapshot_name: &str) {
    assert_expected_returned_tokens!(code, snapshot_name);
}

#[rstest(
    code,
    snapshot_name,
    case("TRUE\tfalse", "test_lexical_analysis_tokenization_for_keywords_case1"),
    case("\t  let\n\r", "test_lexical_analysis_tokenization_for_keywords_case5"),
    case("\t  LET\n\r", "test_lexical_analysis_tokenization_for_keywords_case7")
)]
fn test_lexical_analysis_tokenization_for_keywords(code: &str, snapshot_name: &str) {
    assert_expected_returned_tokens!(code, snapshot_name);
}

#[rstest(
    code,
    snapshot_name,
    case(
        "\t2 == 4\n",
        "test_lexical_analysis_tokenization_for_special_multi_characters_case1"
    ),
    case(
        "\t !=   4",
        "test_lexical_analysis_tokenization_for_special_multi_characters_case2"
    )
)]
fn test_lexical_analysis_tokenization_for_special_multi_characters(
    code: &str,
    snapshot_name: &str,
) {
    assert_expected_returned_tokens!(code, snapshot_name);
}

#[rstest(
    code,
    snapshot_name,
    case(
        "\r! *\t=",
        "test_lexical_analysis_tokenization_for_special_characters_case1"
    ),
    case(
        "\t-/ * ",
        "test_lexical_analysis_tokenization_for_special_characters_case2"
    ),
    case(
        "> < \n * ",
        "test_lexical_analysis_tokenization_for_special_characters_case3"
    ),
    case(
        "=+(){},;",
        "test_lexical_analysis_tokenization_for_special_characters_case4"
    ),
    case(
        " =\n\r+\t    ( )\t\n",
        "test_lexical_analysis_tokenization_for_special_characters_case5"
    )
)]
fn test_lexical_analysis_tokenization_for_special_characters(code: &str, snapshot_name: &str) {
    assert_expected_returned_tokens!(code, snapshot_name);
}

#[test]
fn test_empty_code() {
    assert_expected_returned_tokens!("", "test_empty_code");
}

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
