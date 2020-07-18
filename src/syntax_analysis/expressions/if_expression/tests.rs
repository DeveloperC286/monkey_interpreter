use insta::assert_json_snapshot;
use rstest::rstest;

use crate::lexical_analysis::token::{Token, TokenType};

#[rstest(
    tokens,
    snapshot_name,
    case(
        vec ! [
            Token{token_type: TokenType::IF, literal: "if".to_string()},
            Token{token_type: TokenType::OPENING_ROUND_BRACKET, literal: "(".to_string()},
            Token{token_type: TokenType::IDENTIFIER, literal: "x".to_string()},
            Token{token_type: TokenType::LESSER_THAN, literal: "<".to_string()},
            Token{token_type: TokenType::IDENTIFIER, literal: "y".to_string()},
            Token{token_type: TokenType::CLOSING_ROUND_BRACKET, literal: ")".to_string()},
            Token{token_type: TokenType::OPENING_CURLY_BRACKET, literal: "{".to_string()},
            Token{token_type: TokenType::IDENTIFIER, literal: "x".to_string()},
            Token{token_type: TokenType::CLOSING_CURLY_BRACKET, literal: "}".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_if_expression_case1"
    ),
    case(
        vec ! [
            Token{token_type: TokenType::IF, literal: "if".to_string()},
            Token{token_type: TokenType::OPENING_ROUND_BRACKET, literal: "(".to_string()},
            Token{token_type: TokenType::IDENTIFIER, literal: "x".to_string()},
            Token{token_type: TokenType::LESSER_THAN, literal: "<".to_string()},
            Token{token_type: TokenType::IDENTIFIER, literal: "y".to_string()},
            Token{token_type: TokenType::CLOSING_ROUND_BRACKET, literal: ")".to_string()},
            Token{token_type: TokenType::OPENING_CURLY_BRACKET, literal: "{".to_string()},
            Token{token_type: TokenType::IDENTIFIER, literal: "x".to_string()},
            Token{token_type: TokenType::CLOSING_CURLY_BRACKET, literal: "}".to_string()},
            Token{token_type: TokenType::ELSE, literal: "else".to_string()},
            Token{token_type: TokenType::OPENING_CURLY_BRACKET, literal: "{".to_string()},
            Token{token_type: TokenType::IDENTIFIER, literal: "y".to_string()},
            Token{token_type: TokenType::CLOSING_CURLY_BRACKET, literal: "}".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_if_expression_case2"
    ),
)]
fn test_syntax_analysis_for_if_expression(tokens: Vec<Token>, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(tokens, snapshot_name);
}