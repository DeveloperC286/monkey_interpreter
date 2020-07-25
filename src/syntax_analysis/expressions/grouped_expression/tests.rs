use insta::assert_json_snapshot;
use rstest::rstest;

use crate::lexical_analysis::token::Token;

#[rstest(
    tokens,
    snapshot_name,
    case(
        vec ! [
            Token::MINUS,
            Token::OPENING_ROUND_BRACKET,
            Token::INTEGER{literal: "5".to_string()},
            Token::MULTIPLY,
            Token::INTEGER{literal: "10".to_string()},
            Token::CLOSING_ROUND_BRACKET,
            Token::SEMI_COLON,
            Token::EOF,
        ],
        "test_syntax_analysis_for_grouped_expressions_case1"
    ),
    case(
        vec ! [
            Token::OPENING_ROUND_BRACKET,
            Token::INTEGER{literal: "5".to_string()},
            Token::MINUS,
            Token::INTEGER{literal: "10".to_string()},
            Token::CLOSING_ROUND_BRACKET,
            Token::MULTIPLY,
            Token::INTEGER{literal: "10".to_string()},
            Token::SEMI_COLON,
            Token::EOF,
        ],
        "test_syntax_analysis_for_grouped_expressions_case2"
    ),
    case(
        vec ! [
            Token::INTEGER{literal: "1".to_string()},
            Token::PLUS,
            Token::OPENING_ROUND_BRACKET,
            Token::INTEGER{literal: "2".to_string()},
            Token::PLUS,
            Token::INTEGER{literal: "3".to_string()},
            Token::CLOSING_ROUND_BRACKET,
            Token::PLUS,
            Token::INTEGER{literal: "4".to_string()},
            Token::SEMI_COLON,
            Token::EOF,
        ],
        "test_syntax_analysis_for_grouped_expressions_case3"
    ),
    case(
        vec ! [
            Token::NOT,
            Token::OPENING_ROUND_BRACKET,
            Token::TRUE,
            Token::EQUALS,
            Token::TRUE,
            Token::CLOSING_ROUND_BRACKET,
            Token::SEMI_COLON,
            Token::EOF,
        ],
        "test_syntax_analysis_for_grouped_expressions_case4"
    ),
)]
fn test_syntax_analysis_for_grouped_expressions(tokens: Vec<Token>, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(tokens, snapshot_name);
}
