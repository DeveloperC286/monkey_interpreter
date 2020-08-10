use crate::evaluator::object::Object;
use crate::lexical_analysis::token::Token;

pub fn evaluate(integer_token: Token) -> Object {
    match integer_token {
        Token::INTEGER { literal } => Object::INTEGER {
            value: literal.parse().unwrap(),
        },
        _ => Object::NULL,
    }
}

#[cfg(test)]
mod tests;
