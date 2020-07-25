use insta::assert_json_snapshot;
use rstest::rstest;

use crate::lexical_analysis::token::Token;

#[rstest(
    tokens,
    snapshot_name,
    case(
        vec ! [
            Token::IDENTIFIER{literal: "add".to_string()},
            Token::OPENING_ROUND_BRACKET,
            Token::INTEGER{literal: "5".to_string()},
            Token::COMMA,
            Token::INTEGER{literal: "12".to_string()},
            Token::CLOSING_ROUND_BRACKET,
            Token::EOF,
        ],
        "test_syntax_analysis_for_call_expression_case1"
    ),
    case(
        vec ! [
            Token::IDENTIFIER{literal: "add".to_string()},
            Token::OPENING_ROUND_BRACKET,
            Token::INTEGER{literal: "3".to_string()},
            Token::COMMA,
            Token::INTEGER{literal: "7".to_string()},
            Token::MULTIPLY,
            Token::INTEGER{literal: "2".to_string()},
            Token::CLOSING_ROUND_BRACKET,
            Token::SEMI_COLON,
            Token::EOF,
        ],
        "test_syntax_analysis_for_call_expression_case2"
    ),
)]
fn test_syntax_analysis_for_call_expression(tokens: Vec<Token>, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(tokens, snapshot_name);
}
