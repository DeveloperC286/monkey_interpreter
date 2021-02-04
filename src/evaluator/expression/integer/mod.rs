use crate::evaluator::model::object::Object;
use crate::lexical_analysis::model::token::Token;

pub fn evaluate(integer_token: Token) -> Object {
    match integer_token {
        Token::INTEGER { literal } => Object::INTEGER {
            value: literal.parse().unwrap(),
        },
        _ => panic!("Integer token not a integer token."),
    }
}

#[cfg(test)]
mod tests;
