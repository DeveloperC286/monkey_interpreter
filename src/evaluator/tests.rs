use insta::assert_json_snapshot;
use rstest::rstest;

use super::*;

#[rstest(
    abstract_syntax_tree,
    snapshot_name,
    case(
        AbstractSyntaxTree{
            abstract_syntax_tree: vec![],
            syntax_parsing_errors: vec![],
        },
        "test_evaluator_input_edgecases_case1"
    ),
)]
fn test_evaluator_input_edgecases(abstract_syntax_tree: AbstractSyntaxTree, snapshot_name: &str) {
    assert_expected_returned_object!(abstract_syntax_tree, snapshot_name);
}
