use crate::evaluator::object::Object;
use crate::lexical_analysis::token::Token;

pub fn evaluate(boolean_token: Token) -> Object {
    match boolean_token {
        Token::TRUE => Object::TRUE,
        Token::FALSE => Object::FALSE,
        _ => Object::NULL,
    }
}

#[cfg(test)]
mod tests;
