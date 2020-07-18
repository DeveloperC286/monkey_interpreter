use insta::assert_json_snapshot;
use rstest::rstest;

use crate::lexical_analysis::token::{Token, TokenType};

#[rstest(
    tokens,
    snapshot_name,
    case(
        vec ! [
            Token{token_type: TokenType::IDENTIFIER, literal: "add".to_string()},
            Token{token_type: TokenType::OPENING_ROUND_BRACKET, literal: "(".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "5".to_string()},
            Token{token_type: TokenType::COMMA, literal: ",".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "12".to_string()},
            Token{token_type: TokenType::CLOSING_ROUND_BRACKET, literal: ")".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_call_expression_case1"
    ),
    case(
        vec ! [
            Token{token_type: TokenType::IDENTIFIER, literal: "add".to_string()},
            Token{token_type: TokenType::OPENING_ROUND_BRACKET, literal: "(".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "3".to_string()},
            Token{token_type: TokenType::COMMA, literal: ",".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "7".to_string()},
            Token{token_type: TokenType::MULTIPLY, literal: "*".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "2".to_string()},
            Token{token_type: TokenType::CLOSING_ROUND_BRACKET, literal: ")".to_string()},
            Token{token_type: TokenType::SEMI_COLON, literal: ";".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_call_expression_case2"
    ),
)]
fn test_syntax_analysis_for_call_expression(tokens: Vec<Token>, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(tokens, snapshot_name);
}
