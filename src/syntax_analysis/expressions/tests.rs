use insta::assert_json_snapshot;
use rstest::rstest;

use crate::lexical_analysis::token::Token;

#[rstest(
    tokens,
    snapshot_name,
    case(
        vec ! [
            Token::TRUE,
            Token::SEMI_COLON,
            Token::EOF,
        ],
        "test_syntax_analysis_for_boolean_expression_case1"
    ),
    case(
        vec ! [
            Token::FALSE,
            Token::EOF,
        ],
        "test_syntax_analysis_for_boolean_expression_case2"
    ),
)]
fn test_syntax_analysis_for_boolean_expression(tokens: Vec<Token>, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(tokens, snapshot_name);
}

#[rstest(
    tokens,
    snapshot_name,
    case(
        vec ! [
            Token::NOT,
            Token::EOF,
        ],
        "test_syntax_analysis_for_prefix_expression_syntax_errors_case1"
    ),
    case(
        vec ! [
            Token::MINUS,
            Token::SEMI_COLON,
            Token::EOF,
        ],
        "test_syntax_analysis_for_prefix_expression_syntax_errors_case2"
    ),
)]
fn test_syntax_analysis_for_prefix_expression_syntax_errors(
    tokens: Vec<Token>,
    snapshot_name: &str,
) {
    assert_expected_returned_abstract_syntax_tree!(tokens, snapshot_name);
}

#[rstest(
    tokens,
    snapshot_name,
    case(
        vec ! [
            Token::NOT,
            Token::INTEGER{literal: "15".to_string()},
            Token::EOF,
        ],
        "test_syntax_analysis_for_prefix_expression_case1"
    ),
    case(
        vec ! [
            Token::MINUS,
            Token::INTEGER{literal: "3".to_string()},
            Token::SEMI_COLON,
            Token::EOF,
        ],
        "test_syntax_analysis_for_prefix_expression_case2"
    ),
    case(
        vec ! [
            Token::NOT,
            Token::FALSE,
            Token::EOF,
        ],
        "test_syntax_analysis_for_prefix_expression_case3"
    ),
)]
fn test_syntax_analysis_for_prefix_expression(tokens: Vec<Token>, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(tokens, snapshot_name);
}

#[rstest(
    tokens,
    snapshot_name,
    case(
        vec ! [
            Token::IDENTIFIER{literal: "temp".to_string()},
            Token::SEMI_COLON,
            Token::EOF,
        ],
        "test_syntax_analysis_for_identifier_expression_case1"
    ),
    case(
        vec ! [
            Token::IDENTIFIER{literal: "varX".to_string()},
            Token::EOF,
        ],
        "test_syntax_analysis_for_identifier_expression_case2"
    ),
)]
fn test_syntax_analysis_for_identifier_expression(tokens: Vec<Token>, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(tokens, snapshot_name);
}

#[rstest(
    tokens,
    snapshot_name,
    case(
        vec ! [
            Token::INTEGER{literal: "5".to_string()},
            Token::SEMI_COLON,
            Token::EOF,
        ],
        "test_syntax_analysis_for_integer_expression_case1"
    ),
    case(
        vec ! [
            Token::INTEGER{literal: "12".to_string()},
            Token::EOF,
        ],
        "test_syntax_analysis_for_integer_expression_case2"
    ),
)]
fn test_syntax_analysis_for_integer_expression(tokens: Vec<Token>, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(tokens, snapshot_name);
}
