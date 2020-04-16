use super::super::lexical_analysis::token::{Token, TokenType};
use super::*;

use insta::assert_json_snapshot;
use rstest::rstest;

#[rstest(
    token_stream,
    snapshot_name,
    case(
        vec![
            Token{token_type: TokenType::NOT, literal: "!".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "15".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_prefix_expression_case1"
    ),
    case(
        vec![
            Token{token_type: TokenType::MINUS, literal: "-".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "3".to_string()},
            Token{token_type: TokenType::SEMI_COLON, literal: ";".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_prefix_expression_case2"
    ),
)]
fn test_syntax_analysis_for_prefix_expression(token_stream: Vec<Token>, snapshot_name: &str) {
    //given
    let mut syntax_analysis = SyntaxAnalysis::new(token_stream);

    //when
    let returned_abstract_syntax_tree = syntax_analysis.parse();

    //then
    assert_json_snapshot!(snapshot_name, returned_abstract_syntax_tree);
}

#[rstest(
    token_stream,
    snapshot_name,
    case(
        vec![
            Token{token_type: TokenType::IDENTIFIER, literal: "temp".to_string()},
            Token{token_type: TokenType::SEMI_COLON, literal: ";".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_identifier_expression_case1"
    ),
    case(
        vec![
            Token{token_type: TokenType::IDENTIFIER, literal: "varX".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_identifier_expression_case2"
    ),
)]
fn test_syntax_analysis_for_identifier_expression(token_stream: Vec<Token>, snapshot_name: &str) {
    //given
    let mut syntax_analysis = SyntaxAnalysis::new(token_stream);

    //when
    let returned_abstract_syntax_tree = syntax_analysis.parse();

    //then
    assert_json_snapshot!(snapshot_name, returned_abstract_syntax_tree);
}

#[rstest(
    token_stream,
    snapshot_name,
    case(
        vec![
            Token{token_type: TokenType::INTEGER, literal: "5".to_string()},
            Token{token_type: TokenType::SEMI_COLON, literal: ";".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_integer_expression_case1"
    ),
    case(
        vec![
            Token{token_type: TokenType::INTEGER, literal: "12".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_integer_expression_case2"
    ),
)]
fn test_syntax_analysis_for_integer_expression(token_stream: Vec<Token>, snapshot_name: &str) {
    //given
    let mut syntax_analysis = SyntaxAnalysis::new(token_stream);

    //when
    let returned_abstract_syntax_tree = syntax_analysis.parse();

    //then
    assert_json_snapshot!(snapshot_name, returned_abstract_syntax_tree);
}

#[rstest(
    token_stream,
    snapshot_name,
    case(
        vec![
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
fn test_syntax_analysis_for_combined_statements(token_stream: Vec<Token>, snapshot_name: &str) {
    //given
    let mut syntax_analysis = SyntaxAnalysis::new(token_stream);

    //when
    let returned_abstract_syntax_tree = syntax_analysis.parse();

    //then
    assert_json_snapshot!(snapshot_name, returned_abstract_syntax_tree);
}

#[rstest(
    token_stream,
    snapshot_name,
    case(
        vec![
            Token{token_type: TokenType::RETURN, literal: "RETURN".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "5".to_string()},
            Token{token_type: TokenType::SEMI_COLON, literal: ";".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_return_statements_case1"
    ),
    case(
         vec![
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
fn test_syntax_analysis_for_return_statements(token_stream: Vec<Token>, snapshot_name: &str) {
    //given
    let mut syntax_analysis = SyntaxAnalysis::new(token_stream);

    //when
    let returned_abstract_syntax_tree = syntax_analysis.parse();

    //then
    assert_json_snapshot!(snapshot_name, returned_abstract_syntax_tree);
}

#[rstest(
    token_stream,
    snapshot_name,
    case(
        vec![
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
        vec![
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
fn test_syntax_analysis_for_let_statements(token_stream: Vec<Token>, snapshot_name: &str) {
    //given
    let mut syntax_analysis = SyntaxAnalysis::new(token_stream);

    //when
    let returned_abstract_syntax_tree = syntax_analysis.parse();

    //then
    assert_json_snapshot!(snapshot_name, returned_abstract_syntax_tree);
}
