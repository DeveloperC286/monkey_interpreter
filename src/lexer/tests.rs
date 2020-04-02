use super::*;
use rstest::rstest;
use token::{Token, TokenType};

#[rstest(
    code,
    expected_token_order,
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
fn test_lexer_tokenization_for_special_characters(code: &str, expected_token_order: Vec<Token>) {
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
    let mut lexer = Lexer::new(code.to_string());
    let mut returned_token_order = Vec::new();

    //when
    loop {
        let token = lexer.get_next_token();

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
            panic!("expected_token_order[{}] != returned_token_order[{}]", i, i);
        }
    }
}
