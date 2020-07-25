use insta::assert_json_snapshot;
use rstest::rstest;

use crate::lexical_analysis::token::Token;

#[rstest(
    tokens,
    snapshot_name,
    case(
        vec ! [
            Token::RETURN,
            Token::INTEGER{literal: "5".to_string()},
            Token::SEMI_COLON,
            Token::EOF,
        ],
        "test_syntax_analysis_for_return_statements_case1"
        ),
    case(
        vec ! [
            Token::RETURN,
            Token::INTEGER{literal: "5".to_string()},
            Token::SEMI_COLON,
            Token::RETURN,
            Token::IDENTIFIER{literal: "x".to_string()},
            Token::SEMI_COLON,
            Token::EOF,
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
            Token::LET,
            Token::IDENTIFIER{literal: "x".to_string()},
            Token::ASSIGN,
            Token::INTEGER{literal: "5".to_string()},
            Token::SEMI_COLON,
            Token::EOF,
        ],
        "test_syntax_analysis_for_let_statements_case1"
    ),
    case(
        vec ! [
            Token::LET,
            Token::IDENTIFIER{literal: "x".to_string()},
            Token::ASSIGN,
            Token::INTEGER{literal: "5".to_string()},
            Token::SEMI_COLON,
            Token::LET,
            Token::IDENTIFIER{literal: "z".to_string()},
            Token::ASSIGN,
            Token::INTEGER{literal: "7".to_string()},
            Token::PLUS,
            Token::INTEGER{literal: "10".to_string()},
            Token::SEMI_COLON,
            Token::EOF,
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
            Token::LET,
            Token::IDENTIFIER{literal: "x".to_string()},
            Token::EOF,
        ],
        "test_syntax_analysis_for_let_statements_syntax_errors_case1"
    ),
    case(
        vec ! [
            Token::LET,
            Token::EOF,
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
            Token::LET,
            Token::IDENTIFIER{literal: "x".to_string()},
            Token::ASSIGN,
            Token::INTEGER{literal: "5".to_string()},
            Token::SEMI_COLON,
            Token::EOF,
        ],
        "test_syntax_analysis_for_let_statements_case1",
        vec ! [
            Token::LET,
            Token::IDENTIFIER{literal: "x".to_string()},
            Token::ASSIGN,
            Token::INTEGER{literal: "5".to_string()},
            Token::SEMI_COLON,
            Token::LET,
            Token::IDENTIFIER{literal: "z".to_string()},
            Token::ASSIGN,
            Token::INTEGER{literal: "7".to_string()},
            Token::PLUS,
            Token::INTEGER{literal: "10".to_string()},
            Token::SEMI_COLON,
            Token::EOF,
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
            Token::LET,
            Token::IDENTIFIER{literal: "x".to_string()},
            Token::ASSIGN,
            Token::INTEGER{literal: "5".to_string()},
            Token::SEMI_COLON,
            Token::RETURN,
            Token::INTEGER{literal: "5".to_string()},
            Token::SEMI_COLON,
            Token::EOF,
        ],
        "test_syntax_analysis_for_combined_statements_case1"
    ),
)]
fn test_syntax_analysis_for_combined_statements(tokens: Vec<Token>, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(tokens, snapshot_name);
}
