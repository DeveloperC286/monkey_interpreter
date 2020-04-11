use super::token::{Token, TokenType};
use super::*;
use rstest::rstest;

#[rstest(
    code,
    expected_token_order,
    case(
        "\t123 \n",
        vec![
            Token{token_type: TokenType::INTEGER, literal: "123".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ]
    ),
)]
fn test_lexical_analysis_tokenization_for_integers(code: &str, expected_token_order: Vec<Token>) {
    //when
    let returned_token_order = get_returned_token_order(code);

    //then
    assert_token_orders_equal(expected_token_order, returned_token_order);
}

#[rstest(
    code,
    expected_token_order,
    case(
        "\tlet x;\n",
        vec![
            Token{token_type: TokenType::LET, literal: "let".to_string()},
            Token{token_type: TokenType::IDENTIFIER, literal: "x".to_string()},
            Token{token_type: TokenType::SEMI_COLON, literal: ";".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ]
    ),
    case(
        "let varX\t=  5 +5;",
        vec![
            Token{token_type: TokenType::LET, literal: "let".to_string()},
            Token{token_type: TokenType::IDENTIFIER, literal: "varX".to_string()},
            Token{token_type: TokenType::ASSIGN, literal: "=".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "5".to_string()},
            Token{token_type: TokenType::PLUS, literal: "+".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "5".to_string()},
            Token{token_type: TokenType::SEMI_COLON, literal: ";".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ]
    ),
)]
fn test_lexical_analysis_tokenization_for_identifiers(
    code: &str,
    expected_token_order: Vec<Token>,
) {
    //when
    let returned_token_order = get_returned_token_order(code);

    //then
    assert_token_orders_equal(expected_token_order, returned_token_order);
}

#[rstest(
    code,
    expected_token_order,
    case(
        "TRUE\tfalse",
        vec![
            Token{token_type: TokenType::TRUE, literal: "TRUE".to_string()},
            Token{token_type: TokenType::FALSE, literal: "false".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ]
    ),
    case(
        "if\tfalse\n return 3;",
        vec![
            Token{token_type: TokenType::IF, literal: "if".to_string()},
            Token{token_type: TokenType::FALSE, literal: "false".to_string()},
            Token{token_type: TokenType::RETURN, literal: "return".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "3".to_string()},
            Token{token_type: TokenType::SEMI_COLON, literal: ";".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ]
    ),
    case(
        "\tIF TRUE\n\t\treturn 3;\n\telse\n\t\treturn 45;",
        vec![
            Token{token_type: TokenType::IF, literal: "IF".to_string()},
            Token{token_type: TokenType::TRUE, literal: "TRUE".to_string()},
            Token{token_type: TokenType::RETURN, literal: "return".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "3".to_string()},
            Token{token_type: TokenType::SEMI_COLON, literal: ";".to_string()},
            Token{token_type: TokenType::ELSE, literal: "else".to_string()},
            Token{token_type: TokenType::RETURN, literal: "return".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "45".to_string()},
            Token{token_type: TokenType::SEMI_COLON, literal: ";".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ]
    ),
    case(
        "\n\rfn",
        vec![
            Token{token_type: TokenType::FUNCTION, literal: "fn".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ]
    ),
    case(
        "\t  let\n\r",
        vec![
            Token{token_type: TokenType::LET, literal: "let".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ]
    ),
    case(
        " Fn",
        vec![
            Token{token_type: TokenType::FUNCTION, literal: "Fn".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ]
    ),
    case(
        "\t  LET\n\r",
        vec![
            Token{token_type: TokenType::LET, literal: "LET".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ]
    ),
)]
fn test_lexical_analysis_tokenization_for_keywords(code: &str, expected_token_order: Vec<Token>) {
    //when
    let returned_token_order = get_returned_token_order(code);

    //then
    assert_token_orders_equal(expected_token_order, returned_token_order);
}

#[rstest(
    code,
    expected_token_order,
    case(
        "\t2 == 4\n",
        vec![
            Token{token_type: TokenType::INTEGER, literal: "2".to_string()},
            Token{token_type: TokenType::EQUALS, literal: "==".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "4".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ]
    ),
    case(
        "\t !=   4",
        vec![
            Token{token_type: TokenType::NOT_EQUALS, literal: "!=".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "4".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ]
    ),
)]
fn test_lexical_analysis_tokenization_for_special_multi_characters(
    code: &str,
    expected_token_order: Vec<Token>,
) {
    //when
    let returned_token_order = get_returned_token_order(code);

    //then
    assert_token_orders_equal(expected_token_order, returned_token_order);
}

#[rstest(
    code,
    expected_token_order,
    case(
        "\r! *\t=",
        vec![
            Token{token_type: TokenType::NOT, literal: "!".to_string()},
            Token{token_type: TokenType::MULTIPLY, literal: "*".to_string()},
            Token{token_type: TokenType::ASSIGN, literal: "=".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ]
    ),
    case(
        "\t-/ * ",
        vec![
            Token{token_type: TokenType::MINUS, literal: "-".to_string()},
            Token{token_type: TokenType::DIVIDE, literal: "/".to_string()},
            Token{token_type: TokenType::MULTIPLY, literal: "*".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ]
    ),
    case(
        "> < \n * ",
        vec![
            Token{token_type: TokenType::GREATER_THAN, literal: ">".to_string()},
            Token{token_type: TokenType::LESSER_THAN, literal: "<".to_string()},
            Token{token_type: TokenType::MULTIPLY, literal: "*".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ]
    ),
    case(
        "=+(){},;",
        vec![
            Token{token_type: TokenType::ASSIGN, literal: "=".to_string()},
            Token{token_type: TokenType::PLUS, literal: "+".to_string()},
            Token{token_type: TokenType::OPENING_ROUND_BRACKET, literal: "(".to_string()},
            Token{token_type: TokenType::CLOSING_ROUND_BRACKET, literal: ")".to_string()},
            Token{token_type: TokenType::OPENING_CURLY_BRACKET, literal: "{".to_string()},
            Token{token_type: TokenType::CLOSING_CURLY_BRACKET, literal: "}".to_string()},
            Token{token_type: TokenType::COMMA, literal: ",".to_string()},
            Token{token_type: TokenType::SEMI_COLON, literal: ";".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ]
    ),
    case(
        " =\n\r+\t    ( )\t\n",
        vec![
            Token{token_type: TokenType::ASSIGN, literal: "=".to_string()},
            Token{token_type: TokenType::PLUS, literal: "+".to_string()},
            Token{token_type: TokenType::OPENING_ROUND_BRACKET, literal: "(".to_string()},
            Token{token_type: TokenType::CLOSING_ROUND_BRACKET, literal: ")".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ]
    ),
)]
fn test_lexical_analysis_tokenization_for_special_characters(
    code: &str,
    expected_token_order: Vec<Token>,
) {
    //when
    let returned_token_order = get_returned_token_order(code);

    //then
    assert_token_orders_equal(expected_token_order, returned_token_order);
}

#[test]
fn test_empty_code() {
    //given
    let expected_token_order = vec![Token {
        token_type: TokenType::EOF,
        literal: "".to_string(),
    }];

    //when
    let returned_token_order = get_returned_token_order("");

    //then
    assert_token_orders_equal(expected_token_order, returned_token_order);
}

fn get_returned_token_order(code: &str) -> Vec<Token> {
    let mut lexical_analysis = LexicalAnalysis::new(code.to_string());
    let mut returned_token_order = Vec::new();

    //when
    loop {
        let token = lexical_analysis.get_next_token();

        if token.token_type == TokenType::EOF {
            returned_token_order.push(token);
            break;
        } else {
            returned_token_order.push(token);
        }
    }

    return returned_token_order;
}

fn assert_token_orders_equal(expected_token_order: Vec<Token>, returned_token_order: Vec<Token>) {
    assert_eq!(expected_token_order.len(), returned_token_order.len());

    for i in 0..expected_token_order.len() {
        if expected_token_order[i] != returned_token_order[i] {
            panic!(
                "expected_token_order[{:?}] != returned_token_order[{:?}]",
                expected_token_order[i], returned_token_order[i]
            );
        }
    }
}
