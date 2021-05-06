use crate::evaluator::model::object::Object;
use crate::lexical_analysis::model::token::Token;

pub fn evaluate(boolean_token: Token) -> Object {
    match boolean_token {
        Token::True => Object::True,
        Token::False => Object::False,
        _ => panic!("Boolean token not a boolean token."),
    }
}

#[cfg(test)]
mod tests;
