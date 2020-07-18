use insta::assert_json_snapshot;
use rstest::rstest;

use crate::lexical_analysis::token::{Token, TokenType};

#[rstest(
    tokens,
    snapshot_name,
    case(
        vec ! [
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_tokens_input_edgecases_case1"
    ),
    case(
        vec ! [],
        "test_syntax_analysis_tokens_input_edgecases_case2"
    ),
    case(
        vec ! [
            Token{token_type: TokenType::SEMI_COLON, literal: ";".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_tokens_input_edgecases_case3"
    ),
)]
fn test_syntax_analysis_tokens_input_edgecases(tokens: Vec<Token>, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(tokens, snapshot_name);
}
