use crate::lexical_analysis::model::token::Token;
use crate::syntax_analysis::model::syntax_error::SyntaxError;
use crate::syntax_analysis::model::syntax_tree_node::Block;
use crate::syntax_analysis::SyntaxAnalysis;

impl<'a> SyntaxAnalysis<'a> {
    pub(crate) fn parse_block(&mut self) -> Result<Block, SyntaxError> {
        debug!("Parsing a block.");
        assert_token!(
            self,
            Token::OpeningCurlyBracket,
            Err(SyntaxError::MissingBlockOpeningCurlyBracket)
        );
        let mut blocks = vec![];

        while let Some(token) = self.tokens.peek() {
            match token {
                Token::ClosingCurlyBracket | Token::EndOfFile => break,
                _ => {
                    blocks.push(self.get_next_syntax_tree_node()?);
                }
            }
        }

        assert_token!(
            self,
            Token::ClosingCurlyBracket,
            Err(SyntaxError::MissingBlockClosingCurlyBracket)
        );

        Ok(Block { nodes: blocks })
    }
}
