use insta::assert_json_snapshot;
use rstest::rstest;

use crate::lexical_analysis::token::{Token, TokenType};

#[rstest(
    tokens,
    snapshot_name,
    case(
        vec ! [
            Token{token_type: TokenType::INTEGER, literal: "15".to_string()},
            Token{token_type: TokenType::PLUS, literal: "+".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "5".to_string()},
            Token{token_type: TokenType::SEMI_COLON, literal: ";".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_infix_expression_case1"
    ),
    case(
        vec ! [
            Token{token_type: TokenType::INTEGER, literal: "6".to_string()},
            Token{token_type: TokenType::MINUS, literal: "-".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "6".to_string()},
            Token{token_type: TokenType::SEMI_COLON, literal: ";".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_infix_expression_case2"
    ),
    case(
        vec ! [
            Token{token_type: TokenType::INTEGER, literal: "3".to_string()},
            Token{token_type: TokenType::MULTIPLY, literal: "*".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "4".to_string()},
            Token{token_type: TokenType::SEMI_COLON, literal: ";".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_infix_expression_case3"
    ),
    case(
        vec ! [
            Token{token_type: TokenType::INTEGER, literal: "5".to_string()},
            Token{token_type: TokenType::DIVIDE, literal: "/".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "10".to_string()},
            Token{token_type: TokenType::SEMI_COLON, literal: ";".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_infix_expression_case4"
    ),
    case(
        vec ! [
            Token{token_type: TokenType::MINUS, literal: "-".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "5".to_string()},
            Token{token_type: TokenType::MULTIPLY, literal: "*".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "10".to_string()},
            Token{token_type: TokenType::SEMI_COLON, literal: ";".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_infix_expression_case5"
    ),
    case(
        vec ! [
            Token{token_type: TokenType::INTEGER, literal: "5".to_string()},
            Token{token_type: TokenType::MINUS, literal: "-".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "10".to_string()},
            Token{token_type: TokenType::MULTIPLY, literal: "*".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "10".to_string()},
            Token{token_type: TokenType::SEMI_COLON, literal: ";".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_infix_expression_case6"
    ),
    case(
        vec ! [
            Token{token_type: TokenType::INTEGER, literal: "10".to_string()},
            Token{token_type: TokenType::DIVIDE, literal: "/".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "5".to_string()},
            Token{token_type: TokenType::PLUS, literal: "+".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "1".to_string()},
            Token{token_type: TokenType::SEMI_COLON, literal: ";".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_infix_expression_case7"
    ),
    case(
        vec ! [
            Token{token_type: TokenType::INTEGER, literal: "10".to_string()},
            Token{token_type: TokenType::GREATER_THAN, literal: ">".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "5".to_string()},
            Token{token_type: TokenType::EQUALS, literal: "==".to_string()},
            Token{token_type: TokenType::TRUE, literal: "true".to_string()},
        ],
        "test_syntax_analysis_for_infix_expression_case8"
    ),
    case(
        vec ! [
            Token{token_type: TokenType::INTEGER, literal: "10".to_string()},
            Token{token_type: TokenType::LESSER_THAN, literal: "<".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "5".to_string()},
            Token{token_type: TokenType::NOT_EQUALS, literal: "!=".to_string()},
            Token{token_type: TokenType::FALSE, literal: "false".to_string()},
        ],
        "test_syntax_analysis_for_infix_expression_case9"
    ),
)]
fn test_syntax_analysis_for_infix_expression(tokens: Vec<Token>, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(tokens, snapshot_name);
}