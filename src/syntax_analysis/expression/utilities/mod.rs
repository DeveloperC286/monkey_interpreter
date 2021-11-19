use crate::lexical_analysis::model::token::Token;
use crate::syntax_analysis::model::abstract_syntax_tree::syntax_tree_node::Block;
use crate::syntax_analysis::SyntaxAnalysis;

impl<'a> SyntaxAnalysis<'a> {
    pub(crate) fn parse_block(&mut self) -> Option<Block> {
        debug!("Parsing a block.");
        assert_token!(self, Token::OpeningCurlyBracket, None);
        let mut blocks = vec![];

        while let Some(token) = self.tokens.peek() {
            match token {
                Token::ClosingCurlyBracket | Token::EndOfFile => break,
                _ => {
                    if let Some(token) = self.get_next_syntax_tree_node() {
                        blocks.push(token)
                    }
                }
            }
        }

        assert_token!(self, Token::ClosingCurlyBracket, None);

        Some(Block { nodes: blocks })
    }
}
