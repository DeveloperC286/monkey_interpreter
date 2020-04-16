use super::*;
use super::super::lexical_analysis::token::{Token, TokenType};

use rstest::rstest;
use insta::assert_json_snapshot;

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
fn test_syntax_analysis_for_return_statements(token_stream: Vec<Token>, snapshot_name: &str ) {
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
fn test_syntax_analysis_for_let_statements(token_stream: Vec<Token>, snapshot_name: &str ) {
    //given
    let mut syntax_analysis = SyntaxAnalysis::new(token_stream);

    //when
    let returned_abstract_syntax_tree = syntax_analysis.parse();

    //then
    assert_json_snapshot!(snapshot_name, returned_abstract_syntax_tree);
}
