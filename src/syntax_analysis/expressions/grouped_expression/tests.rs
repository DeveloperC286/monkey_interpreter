use insta::assert_json_snapshot;
use rstest::rstest;

use crate::lexical_analysis::token::{Token, TokenType};

#[rstest(
    tokens,
    snapshot_name,
    case(
        vec ! [
            Token{token_type: TokenType::MINUS, literal: "-".to_string()},
            Token{token_type: TokenType::OPENING_ROUND_BRACKET, literal: "(".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "5".to_string()},
            Token{token_type: TokenType::MULTIPLY, literal: "*".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "10".to_string()},
            Token{token_type: TokenType::CLOSING_ROUND_BRACKET, literal: ")".to_string()},
            Token{token_type: TokenType::SEMI_COLON, literal: ";".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_grouped_expressions_case1"
    ),
    case(
        vec ! [
            Token{token_type: TokenType::OPENING_ROUND_BRACKET, literal: "(".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "5".to_string()},
            Token{token_type: TokenType::MINUS, literal: "-".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "10".to_string()},
            Token{token_type: TokenType::CLOSING_ROUND_BRACKET, literal: ")".to_string()},
            Token{token_type: TokenType::MULTIPLY, literal: "*".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "10".to_string()},
            Token{token_type: TokenType::SEMI_COLON, literal: ";".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_grouped_expressions_case2"
    ),
    case(
        vec ! [
            Token{token_type: TokenType::INTEGER, literal: "1".to_string()},
            Token{token_type: TokenType::PLUS, literal: "+".to_string()},
            Token{token_type: TokenType::OPENING_ROUND_BRACKET, literal: "(".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "2".to_string()},
            Token{token_type: TokenType::PLUS, literal: "+".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "3".to_string()},
            Token{token_type: TokenType::CLOSING_ROUND_BRACKET, literal: ")".to_string()},
            Token{token_type: TokenType::PLUS, literal: "+".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "4".to_string()},
            Token{token_type: TokenType::SEMI_COLON, literal: ";".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_grouped_expressions_case3"
    ),
    case(
        vec ! [
            Token{token_type: TokenType::NOT, literal: "!".to_string()},
            Token{token_type: TokenType::OPENING_ROUND_BRACKET, literal: "(".to_string()},
            Token{token_type: TokenType::TRUE, literal: "TRUE".to_string()},
            Token{token_type: TokenType::EQUALS, literal: "==".to_string()},
            Token{token_type: TokenType::TRUE, literal: "TRUE".to_string()},
            Token{token_type: TokenType::CLOSING_ROUND_BRACKET, literal: ")".to_string()},
            Token{token_type: TokenType::SEMI_COLON, literal: ";".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_grouped_expressions_case4"
    ),
)]
fn test_syntax_analysis_for_grouped_expressions(tokens: Vec<Token>, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(tokens, snapshot_name);
}
