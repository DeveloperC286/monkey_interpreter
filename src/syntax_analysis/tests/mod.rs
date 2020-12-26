use rstest::rstest;

#[rstest(
    code,
    snapshot_name,
    case("", "test_syntax_analysis_tokens_input_edgecases_case1"),
    case(";", "test_syntax_analysis_tokens_input_edgecases_case2")
)]
fn test_syntax_analysis_tokens_input_edgecases(code: &str, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(code, snapshot_name)
}
