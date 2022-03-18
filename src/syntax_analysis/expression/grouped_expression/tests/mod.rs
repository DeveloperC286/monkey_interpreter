use rstest::rstest;

#[rstest(
    code,
    snapshot_name,
    case(
        "!(true == true);",
        "test_syntax_analysis_for_grouped_expressions_case4"
    )
)]
fn test_syntax_analysis_for_grouped_expressions(code: &str, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(code, snapshot_name);
}
