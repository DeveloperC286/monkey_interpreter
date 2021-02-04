use std::iter::Peekable;
use std::slice::Iter;

use crate::lexical_analysis::model::token::Token;

pub struct SyntaxAnalysisContext<'a> {
    pub tokens: Peekable<Iter<'a, Token>>,
    pub syntax_parsing_errors: Vec<String>,
}

impl<'a> SyntaxAnalysisContext<'a> {
    pub fn new(tokens: &'a [Token]) -> SyntaxAnalysisContext<'a> {
        SyntaxAnalysisContext {
            tokens: tokens.iter().peekable(),
            syntax_parsing_errors: vec![],
        }
    }
}
