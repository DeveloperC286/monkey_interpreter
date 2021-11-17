use std::iter::Peekable;
use std::slice::Iter;

use crate::lexical_analysis::model::token::Token;

pub(crate) struct SyntaxAnalysisContext<'a> {
    pub(crate) tokens: Peekable<Iter<'a, Token>>,
    pub(crate) syntax_parsing_errors: Vec<String>,
}

impl<'a> SyntaxAnalysisContext<'a> {
    pub(crate) fn new(tokens: &'a [Token]) -> SyntaxAnalysisContext<'a> {
        SyntaxAnalysisContext {
            tokens: tokens.iter().peekable(),
            syntax_parsing_errors: vec![],
        }
    }
}
