use insta::assert_json_snapshot;
use rstest::rstest;

#[rstest(
    code,
    snapshot_name,
    case(
        "-(5 * 10);".to_string(),
        "test_syntax_analysis_for_grouped_expressions_case1"
    ),
    case(
        "(5 - 10) * 10;".to_string(),
        "test_syntax_analysis_for_grouped_expressions_case2"
    ),
    case(
        "1 + (2 + 3) + 4;".to_string(),
        "test_syntax_analysis_for_grouped_expressions_case3"
    ),
    case(
        "!(true == true);".to_string(),
        "test_syntax_analysis_for_grouped_expressions_case4"
    ),
)]
fn test_syntax_analysis_for_grouped_expressions(code: String, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(code, snapshot_name);
}
