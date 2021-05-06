use crate::evaluator::model::object::Object;
use crate::lexical_analysis::model::token::Token;

pub fn evaluate(boolean_token: Token) -> Object {
    match boolean_token {
        Token::True => Object::TRUE,
        Token::False => Object::FALSE,
        _ => panic!("Boolean token not a boolean token."),
    }
}

#[cfg(test)]
mod tests;
