use insta::assert_json_snapshot;
use rstest::rstest;

use crate::lexical_analysis::token::Token;

#[rstest(
    tokens,
    snapshot_name,
    case(
        vec ! [
            Token::IF,
            Token::OPENING_ROUND_BRACKET,
            Token::IDENTIFIER{literal: "x".to_string()},
            Token::LESSER_THAN,
            Token::IDENTIFIER{literal: "y".to_string()},
            Token::CLOSING_ROUND_BRACKET,
            Token::OPENING_CURLY_BRACKET,
            Token::IDENTIFIER{literal: "x".to_string()},
            Token::CLOSING_CURLY_BRACKET,
            Token::EOF,
        ],
        "test_syntax_analysis_for_if_expression_case1"
    ),
    case(
        vec ! [
            Token::IF,
            Token::OPENING_ROUND_BRACKET,
            Token::IDENTIFIER{literal: "x".to_string()},
            Token::LESSER_THAN,
            Token::IDENTIFIER{literal: "y".to_string()},
            Token::CLOSING_ROUND_BRACKET,
            Token::OPENING_CURLY_BRACKET,
            Token::IDENTIFIER{literal: "x".to_string()},
            Token::CLOSING_CURLY_BRACKET,
            Token::ELSE,
            Token::OPENING_CURLY_BRACKET,
            Token::IDENTIFIER{literal: "y".to_string()},
            Token::CLOSING_CURLY_BRACKET,
            Token::EOF,
        ],
        "test_syntax_analysis_for_if_expression_case2"
    ),
)]
fn test_syntax_analysis_for_if_expression(tokens: Vec<Token>, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(tokens, snapshot_name);
}
