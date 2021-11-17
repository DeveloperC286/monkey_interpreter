use crate::lexical_analysis::model::token::Token;
use crate::syntax_analysis::model::abstract_syntax_tree::syntax_tree_node::Block;
use crate::syntax_analysis::model::syntax_analysis_context::SyntaxAnalysisContext;

pub(crate) fn parse_block(
    mut syntax_analysis_context: SyntaxAnalysisContext,
) -> (SyntaxAnalysisContext, Option<Block>) {
    debug!("Parsing a block.");
    assert_token!(syntax_analysis_context, Token::OpeningCurlyBracket, None);
    let mut blocks = vec![];

    while let Some(token) = syntax_analysis_context.tokens.peek() {
        match token {
            Token::ClosingCurlyBracket | Token::EndOfFile => break,
            _ => match crate::syntax_analysis::get_next_syntax_tree_node(syntax_analysis_context) {
                (returned_syntax_analysis_context, Some(token)) => {
                    syntax_analysis_context = returned_syntax_analysis_context;
                    blocks.push(token)
                }
                (returned_syntax_analysis_context, None) => {
                    syntax_analysis_context = returned_syntax_analysis_context;
                }
            },
        }
    }

    assert_token!(syntax_analysis_context, Token::ClosingCurlyBracket, None);

    (syntax_analysis_context, Some(Block { nodes: blocks }))
}
