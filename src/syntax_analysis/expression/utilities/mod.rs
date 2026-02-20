use crate::lexical_analysis::model::token::Token;
use crate::syntax_analysis::model::syntax_tree_node::Block;
use crate::syntax_analysis::SyntaxAnalysis;

impl SyntaxAnalysis<'_> {
    pub(crate) fn parse_block(&mut self) -> anyhow::Result<Block> {
        debug!("Parsing a block.");
        assert_token!(
            self,
            Token::OpeningCurlyBracket,
            "A block must start with a OpeningCurlyBracket token."
        );
        let mut blocks = vec![];

        while let Some(token) = self.tokens.peek() {
            match token {
                Token::ClosingCurlyBracket => break,
                _ => {
                    blocks.push(self.get_next_syntax_tree_node()?);
                }
            }
        }

        assert_token!(
            self,
            Token::ClosingCurlyBracket,
            "A block must end with a ClosingCurlyBracket token."
        );

        Ok(Block { nodes: blocks })
    }
}
