use insta::assert_json_snapshot;
use rstest::rstest;

use crate::lexical_analysis::token::{Token, TokenType};

#[rstest(
    tokens,
    snapshot_name,
    case(
        vec ! [
            Token{token_type: TokenType::RETURN, literal: "RETURN".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "5".to_string()},
            Token{token_type: TokenType::SEMI_COLON, literal: ";".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_return_statements_case1"
        ),
    case(
        vec ! [
            Token{token_type: TokenType::RETURN, literal: "RETURN".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "5".to_string()},
            Token{token_type: TokenType::SEMI_COLON, literal: ";".to_string()},
            Token{token_type: TokenType::RETURN, literal: "return".to_string()},
            Token{token_type: TokenType::IDENTIFIER, literal: "x".to_string()},
            Token{token_type: TokenType::SEMI_COLON, literal: ";".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_return_statements_case2"
    ),
)]
fn test_syntax_analysis_for_return_statements(tokens: Vec<Token>, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(tokens, snapshot_name);
}

#[rstest(
    tokens,
    snapshot_name,
    case(
        vec ! [
            Token{token_type: TokenType::LET, literal: "let".to_string()},
            Token{token_type: TokenType::IDENTIFIER, literal: "x".to_string()},
            Token{token_type: TokenType::ASSIGN, literal: "=".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "5".to_string()},
            Token{token_type: TokenType::SEMI_COLON, literal: ";".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_let_statements_case1"
    ),
    case(
        vec ! [
            Token{token_type: TokenType::LET, literal: "let".to_string()},
            Token{token_type: TokenType::IDENTIFIER, literal: "x".to_string()},
            Token{token_type: TokenType::ASSIGN, literal: "=".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "5".to_string()},
            Token{token_type: TokenType::SEMI_COLON, literal: ";".to_string()},
            Token{token_type: TokenType::LET, literal: "let".to_string()},
            Token{token_type: TokenType::IDENTIFIER, literal: "z".to_string()},
            Token{token_type: TokenType::ASSIGN, literal: "=".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "7".to_string()},
            Token{token_type: TokenType::PLUS, literal: "+".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "10".to_string()},
            Token{token_type: TokenType::SEMI_COLON, literal: ";".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_let_statements_case2"
    ),
)]
fn test_syntax_analysis_for_let_statements(tokens: Vec<Token>, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(tokens, snapshot_name);
}

#[rstest(
    tokens,
    snapshot_name,
    case(
        vec ! [
            Token{token_type: TokenType::LET, literal: "let".to_string()},
            Token{token_type: TokenType::IDENTIFIER, literal: "x".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_let_statements_syntax_errors_case1"
    ),
    case(
        vec ! [
            Token{token_type: TokenType::LET, literal: "let".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_let_statements_syntax_errors_case2"
    ),
)]
fn test_syntax_analysis_for_let_statements_syntax_errors(tokens: Vec<Token>, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(tokens, snapshot_name);
}

#[rstest(
    tokens_1,
    snapshot_name_1,
    tokens_2,
    snapshot_name_2,
    case(
        vec ! [
            Token{token_type: TokenType::LET, literal: "let".to_string()},
            Token{token_type: TokenType::IDENTIFIER, literal: "x".to_string()},
            Token{token_type: TokenType::ASSIGN, literal: "=".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "5".to_string()},
            Token{token_type: TokenType::SEMI_COLON, literal: ";".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_let_statements_case1",
        vec ! [
            Token{token_type: TokenType::LET, literal: "let".to_string()},
            Token{token_type: TokenType::IDENTIFIER, literal: "x".to_string()},
            Token{token_type: TokenType::ASSIGN, literal: "=".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "5".to_string()},
            Token{token_type: TokenType::SEMI_COLON, literal: ";".to_string()},
            Token{token_type: TokenType::LET, literal: "let".to_string()},
            Token{token_type: TokenType::IDENTIFIER, literal: "z".to_string()},
            Token{token_type: TokenType::ASSIGN, literal: "=".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "7".to_string()},
            Token{token_type: TokenType::PLUS, literal: "+".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "10".to_string()},
            Token{token_type: TokenType::SEMI_COLON, literal: ";".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_let_statements_case2"
    ),
)]
fn test_syntax_analysis_for_successive_parsing(
    tokens_1: Vec<Token>,
    snapshot_name_1: &str,
    tokens_2: Vec<Token>,
    snapshot_name_2: &str,
) {
    assert_expected_returned_abstract_syntax_tree!(tokens_1, snapshot_name_1);
    assert_expected_returned_abstract_syntax_tree!(tokens_2, snapshot_name_2);
}

#[rstest(
    tokens,
    snapshot_name,
    case(
        vec ! [
            Token{token_type: TokenType::LET, literal: "let".to_string()},
            Token{token_type: TokenType::IDENTIFIER, literal: "x".to_string()},
            Token{token_type: TokenType::ASSIGN, literal: "=".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "5".to_string()},
            Token{token_type: TokenType::SEMI_COLON, literal: ";".to_string()},
            Token{token_type: TokenType::RETURN, literal: "RETURN".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "5".to_string()},
            Token{token_type: TokenType::SEMI_COLON, literal: ";".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_combined_statements_case1"
    ),
)]
fn test_syntax_analysis_for_combined_statements(tokens: Vec<Token>, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(tokens, snapshot_name);
}
