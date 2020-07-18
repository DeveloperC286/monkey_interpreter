use insta::assert_json_snapshot;
use rstest::rstest;

use crate::lexical_analysis::token::{Token, TokenType};

#[rstest(
    tokens,
    snapshot_name,
    case(
        vec ! [
            Token{token_type: TokenType::FUNCTION, literal: "fn".to_string()},
            Token{token_type: TokenType::OPENING_ROUND_BRACKET, literal: "(".to_string()},
            Token{token_type: TokenType::IDENTIFIER, literal: "x".to_string()},
            Token{token_type: TokenType::COMMA, literal: ",".to_string()},
            Token{token_type: TokenType::IDENTIFIER, literal: "y".to_string()},
            Token{token_type: TokenType::CLOSING_ROUND_BRACKET, literal: ")".to_string()},
            Token{token_type: TokenType::OPENING_CURLY_BRACKET, literal: "{".to_string()},
            Token{token_type: TokenType::IDENTIFIER, literal: "x".to_string()},
            Token{token_type: TokenType::PLUS, literal: "+".to_string()},
            Token{token_type: TokenType::IDENTIFIER, literal: "y".to_string()},
            Token{token_type: TokenType::SEMI_COLON, literal: ";".to_string()},
            Token{token_type: TokenType::CLOSING_CURLY_BRACKET, literal: "}".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_function_expression_case1"
    ),
    case(
        vec ! [
            Token{token_type: TokenType::FUNCTION, literal: "fn".to_string()},
            Token{token_type: TokenType::OPENING_ROUND_BRACKET, literal: "(".to_string()},
            Token{token_type: TokenType::CLOSING_ROUND_BRACKET, literal: ")".to_string()},
            Token{token_type: TokenType::OPENING_CURLY_BRACKET, literal: "{".to_string()},
            Token{token_type: TokenType::CLOSING_CURLY_BRACKET, literal: "}".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_function_expression_case2"
    ),
    case(
        vec ! [
            Token{token_type: TokenType::FUNCTION, literal: "fn".to_string()},
            Token{token_type: TokenType::OPENING_ROUND_BRACKET, literal: "(".to_string()},
            Token{token_type: TokenType::IDENTIFIER, literal: "x".to_string()},
            Token{token_type: TokenType::CLOSING_ROUND_BRACKET, literal: ")".to_string()},
            Token{token_type: TokenType::OPENING_CURLY_BRACKET, literal: "{".to_string()},
            Token{token_type: TokenType::CLOSING_CURLY_BRACKET, literal: "}".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_function_expression_case3"
    ),
)]
fn test_syntax_analysis_for_function_expression(tokens: Vec<Token>, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(tokens, snapshot_name);
}