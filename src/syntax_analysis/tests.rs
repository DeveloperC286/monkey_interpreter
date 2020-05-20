use super::super::lexical_analysis::token::{Token, TokenType};
use super::*;

use insta::assert_json_snapshot;
use rstest::rstest;

#[rstest(
    tokens,
    snapshot_name,
    case(
        vec![
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
)]
fn test_syntax_analysis_for_if_expression(tokens: Vec<Token>, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree(tokens, snapshot_name);
}

#[rstest(
    tokens,
    snapshot_name,
    case(
        vec![
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
        vec![
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
        vec![
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
        vec![
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
    assert_expected_returned_abstract_syntax_tree(tokens, snapshot_name);
}

#[rstest(
    tokens,
    snapshot_name,
    case(
        vec![
            Token{token_type: TokenType::TRUE, literal: "TRUE".to_string()},
            Token{token_type: TokenType::SEMI_COLON, literal: ";".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_boolean_expression_case1"
    ),
    case(
        vec![
            Token{token_type: TokenType::FALSE, literal: "false".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_boolean_expression_case2"
    ),
)]
fn test_syntax_analysis_for_boolean_expression(tokens: Vec<Token>, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree(tokens, snapshot_name);
}

#[rstest(
    tokens_1,
    snapshot_name_1,
    tokens_2,
    snapshot_name_2,
    case(
        vec![
            Token{token_type: TokenType::LET, literal: "let".to_string()},
            Token{token_type: TokenType::IDENTIFIER, literal: "x".to_string()},
            Token{token_type: TokenType::ASSIGN, literal: "=".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "5".to_string()},
            Token{token_type: TokenType::SEMI_COLON, literal: ";".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_let_statements_case1",
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
fn test_syntax_analysis_for_successive_parsing(
    tokens_1: Vec<Token>,
    snapshot_name_1: &str,
    tokens_2: Vec<Token>,
    snapshot_name_2: &str,
) {
    assert_expected_returned_abstract_syntax_tree(tokens_1, snapshot_name_1);
    assert_expected_returned_abstract_syntax_tree(tokens_2, snapshot_name_2);
}

#[rstest(
    tokens,
    snapshot_name,
    case(
        vec![
            Token{token_type: TokenType::INTEGER, literal: "15".to_string()},
            Token{token_type: TokenType::PLUS, literal: "+".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "5".to_string()},
            Token{token_type: TokenType::SEMI_COLON, literal: ";".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_infix_expression_case1"
    ),
    case(
        vec![
            Token{token_type: TokenType::INTEGER, literal: "6".to_string()},
            Token{token_type: TokenType::MINUS, literal: "-".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "6".to_string()},
            Token{token_type: TokenType::SEMI_COLON, literal: ";".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_infix_expression_case2"
    ),
    case(
        vec![
            Token{token_type: TokenType::INTEGER, literal: "3".to_string()},
            Token{token_type: TokenType::MULTIPLY, literal: "*".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "4".to_string()},
            Token{token_type: TokenType::SEMI_COLON, literal: ";".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_infix_expression_case3"
    ),
    case(
        vec![
            Token{token_type: TokenType::INTEGER, literal: "5".to_string()},
            Token{token_type: TokenType::DIVIDE, literal: "/".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "10".to_string()},
            Token{token_type: TokenType::SEMI_COLON, literal: ";".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_infix_expression_case4"
    ),
    case(
        vec![
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
        vec![
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
        vec![
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
        vec![
            Token{token_type: TokenType::INTEGER, literal: "10".to_string()},
            Token{token_type: TokenType::GREATER_THAN, literal: ">".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "5".to_string()},
            Token{token_type: TokenType::EQUALS, literal: "==".to_string()},
            Token{token_type: TokenType::TRUE, literal: "true".to_string()},
        ],
        "test_syntax_analysis_for_infix_expression_case8"
    ),
    case(
        vec![
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
    assert_expected_returned_abstract_syntax_tree(tokens, snapshot_name);
}

#[rstest(
    tokens,
    snapshot_name,
    case(
        vec![
            Token{token_type: TokenType::NOT, literal: "!".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_prefix_expression_syntax_errors_case1"
    ),
    case(
        vec![
            Token{token_type: TokenType::MINUS, literal: "-".to_string()},
            Token{token_type: TokenType::SEMI_COLON, literal: ";".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_prefix_expression_syntax_errors_case2"
    ),
)]
fn test_syntax_analysis_for_prefix_expression_syntax_errors(
    tokens: Vec<Token>,
    snapshot_name: &str,
) {
    assert_expected_returned_abstract_syntax_tree(tokens, snapshot_name);
}

#[rstest(
    tokens,
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
    case(
        vec![
            Token{token_type: TokenType::NOT, literal: "!".to_string()},
            Token{token_type: TokenType::FALSE, literal: "false".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_prefix_expression_case3"
    ),
)]
fn test_syntax_analysis_for_prefix_expression(tokens: Vec<Token>, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree(tokens, snapshot_name);
}

#[rstest(
    tokens,
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
fn test_syntax_analysis_for_identifier_expression(tokens: Vec<Token>, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree(tokens, snapshot_name);
}

#[rstest(
    tokens,
    snapshot_name,
    case(
        vec![
            Token{token_type: TokenType::SEMI_COLON, literal: "%".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_expression_syntax_errors_case1"
    ),
)]
fn test_syntax_analysis_for_expression_syntax_errors(tokens: Vec<Token>, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree(tokens, snapshot_name);
}

#[rstest(
    tokens,
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
fn test_syntax_analysis_for_integer_expression(tokens: Vec<Token>, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree(tokens, snapshot_name);
}

#[rstest(
    tokens,
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
fn test_syntax_analysis_for_combined_statements(tokens: Vec<Token>, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree(tokens, snapshot_name);
}

#[rstest(
    tokens,
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
fn test_syntax_analysis_for_return_statements(tokens: Vec<Token>, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree(tokens, snapshot_name);
}

#[rstest(
    tokens,
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
fn test_syntax_analysis_for_let_statements(tokens: Vec<Token>, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree(tokens, snapshot_name);
}

#[rstest(
    tokens,
    snapshot_name,
    case(
        vec![
            Token{token_type: TokenType::LET, literal: "let".to_string()},
            Token{token_type: TokenType::IDENTIFIER, literal: "x".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_let_statements_syntax_errors_case1"
    ),
    case(
        vec![
            Token{token_type: TokenType::LET, literal: "let".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_for_let_statements_syntax_errors_case2"
    ),
)]
fn test_syntax_analysis_for_let_statements_syntax_errors(tokens: Vec<Token>, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree(tokens, snapshot_name);
}

#[rstest(
    tokens,
    snapshot_name,
    case(
        vec![
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        "test_syntax_analysis_tokens_input_edgecases_case1"
    ),
    case(
        vec![],
        "test_syntax_analysis_tokens_input_edgecases_case2"
    ),
)]
fn test_syntax_analysis_tokens_input_edgecases(tokens: Vec<Token>, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree(tokens, snapshot_name);
}

fn assert_expected_returned_abstract_syntax_tree(tokens: Vec<Token>, snapshot_name: &str) {
    //when
    let returned_abstract_syntax_tree = SyntaxAnalysis::get_abstract_syntax_tree(tokens);

    //then
    assert_json_snapshot!(snapshot_name, returned_abstract_syntax_tree);
}
