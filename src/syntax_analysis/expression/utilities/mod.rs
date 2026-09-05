use log::debug;

use crate::lexical_analysis::model::token::Token;
use crate::syntax_analysis::SyntaxAnalysis;
use crate::syntax_analysis::model::expression_precedence::ExpressionPrecedence;
use crate::syntax_analysis::model::syntax_tree_node::{Block, Expression};

impl SyntaxAnalysis<'_> {
    pub(super) fn parse_block(&mut self) -> anyhow::Result<Block> {
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

    pub(super) fn parse_comma_separated_list<T>(
        &mut self,
        context: &str,
        map_expression: impl Fn(Expression) -> anyhow::Result<T>,
    ) -> anyhow::Result<Vec<T>> {
        debug!("Parsing a {context}.");

        assert_token!(
            self,
            Token::OpeningRoundBracket,
            format!("A {context} must start with a OpeningRoundBracket token.")
        );
        let mut list = vec![];

        if let Some(token) = self.tokens.peek()
            && **token != Token::ClosingRoundBracket
        {
            loop {
                let expression = self.get_expression(ExpressionPrecedence::Lowest)?;
                list.push(map_expression(expression)?);

                match self.tokens.peek() {
                    Some(token) => match token {
                        Token::ClosingRoundBracket => break,
                        Token::Comma => {
                            self.tokens.next();
                        }
                        _ => {
                            anyhow::bail!("A {context} must be comma separated.");
                        }
                    },
                    None => {
                        anyhow::bail!("A {context} ended abruptly.");
                    }
                }
            }
        }

        assert_token!(
            self,
            Token::ClosingRoundBracket,
            format!("A {context} must end with a ClosingRoundBracket token.")
        );
        Ok(list)
    }
}
