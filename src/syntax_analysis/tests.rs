use insta::assert_json_snapshot;
use rstest::rstest;

use crate::lexical_analysis::token::Token;

#[rstest(
    tokens,
    snapshot_name,
    case(
        vec ! [
            Token::EOF,
        ],
        "test_syntax_analysis_tokens_input_edgecases_case1"
    ),
    case(
        vec ! [],
        "test_syntax_analysis_tokens_input_edgecases_case2"
    ),
    case(
        vec ! [
            Token::SEMI_COLON,
            Token::EOF,
        ],
        "test_syntax_analysis_tokens_input_edgecases_case3"
    ),
)]
fn test_syntax_analysis_tokens_input_edgecases(tokens: Vec<Token>, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(tokens, snapshot_name);
}
