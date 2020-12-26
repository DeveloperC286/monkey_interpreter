use rstest::rstest;

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
    case(
        "\tlet x;\n",
        "test_lexical_analysis_tokenization_for_identifiers_case1"
    ),
    case(
        "let varX\t=  5 +5;",
        "test_lexical_analysis_tokenization_for_identifiers_case2"
    ),
    case(
        "\tlet camel_case;\n",
        "test_lexical_analysis_tokenization_for_identifiers_case3"
    ),
    case(
        "\tlet _unused_var;\n",
        "test_lexical_analysis_tokenization_for_identifiers_case4"
    )
)]
fn test_lexical_analysis_tokenization_for_identifiers(code: &str, snapshot_name: &str) {
    assert_expected_returned_tokens!(code, snapshot_name);
}

#[rstest(
    code,
    snapshot_name,
    case("TRUE\tfalse", "test_lexical_analysis_tokenization_for_keywords_case1"),
    case(
        "if\tfalse\n return 3;",
        "test_lexical_analysis_tokenization_for_keywords_case2"
    ),
    case(
        "\tIF TRUE\n\t\treturn 3;\n\telse\n\t\treturn 45;",
        "test_lexical_analysis_tokenization_for_keywords_case3"
    ),
    case("\n\rfn", "test_lexical_analysis_tokenization_for_keywords_case4"),
    case("\t  let\n\r", "test_lexical_analysis_tokenization_for_keywords_case5"),
    case(" Fn", "test_lexical_analysis_tokenization_for_keywords_case6"),
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
