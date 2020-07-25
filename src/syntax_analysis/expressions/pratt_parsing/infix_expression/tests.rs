use insta::assert_json_snapshot;
use rstest::rstest;

use crate::lexical_analysis::token::Token;

#[rstest(
    tokens,
    snapshot_name,
    case(
        vec ! [
            Token::INTEGER{literal: "15".to_string()},
            Token::PLUS,
            Token::INTEGER{literal: "5".to_string()},
            Token::SEMI_COLON,
            Token::EOF,
        ],
        "test_syntax_analysis_for_infix_expression_case1"
    ),
    case(
        vec ! [
            Token::INTEGER{literal: "6".to_string()},
            Token::MINUS,
            Token::INTEGER{literal: "6".to_string()},
            Token::SEMI_COLON,
            Token::EOF,
        ],
        "test_syntax_analysis_for_infix_expression_case2"
    ),
    case(
        vec ! [
            Token::INTEGER{literal: "3".to_string()},
            Token::MULTIPLY,
            Token::INTEGER{literal: "4".to_string()},
            Token::SEMI_COLON,
            Token::EOF,
        ],
        "test_syntax_analysis_for_infix_expression_case3"
    ),
    case(
        vec ! [
            Token::INTEGER{literal: "5".to_string()},
            Token::DIVIDE,
            Token::INTEGER{literal: "10".to_string()},
            Token::SEMI_COLON,
            Token::EOF,
        ],
        "test_syntax_analysis_for_infix_expression_case4"
    ),
    case(
        vec ! [
            Token::MINUS,
            Token::INTEGER{literal: "5".to_string()},
            Token::MULTIPLY,
            Token::INTEGER{literal: "10".to_string()},
            Token::SEMI_COLON,
            Token::EOF,
        ],
        "test_syntax_analysis_for_infix_expression_case5"
    ),
    case(
        vec ! [
            Token::INTEGER{literal: "5".to_string()},
            Token::MINUS,
            Token::INTEGER{literal: "10".to_string()},
            Token::MULTIPLY,
            Token::INTEGER{literal: "10".to_string()},
            Token::SEMI_COLON,
            Token::EOF,
        ],
        "test_syntax_analysis_for_infix_expression_case6"
    ),
    case(
        vec ! [
            Token::INTEGER{literal: "10".to_string()},
            Token::DIVIDE,
            Token::INTEGER{literal: "5".to_string()},
            Token::PLUS,
            Token::INTEGER{literal: "1".to_string()},
            Token::SEMI_COLON,
            Token::EOF,
        ],
        "test_syntax_analysis_for_infix_expression_case7"
    ),
    case(
        vec ! [
            Token::INTEGER{literal: "10".to_string()},
            Token::GREATER_THAN,
            Token::INTEGER{literal: "5".to_string()},
            Token::EQUALS,
            Token::TRUE,
        ],
        "test_syntax_analysis_for_infix_expression_case8"
    ),
    case(
        vec ! [
            Token::INTEGER{literal: "10".to_string()},
            Token::LESSER_THAN,
            Token::INTEGER{literal: "5".to_string()},
            Token::NOT_EQUALS,
            Token::FALSE,
        ],
        "test_syntax_analysis_for_infix_expression_case9"
    ),
)]
fn test_syntax_analysis_for_infix_expression(tokens: Vec<Token>, snapshot_name: &str) {
    assert_expected_returned_abstract_syntax_tree!(tokens, snapshot_name);
}
