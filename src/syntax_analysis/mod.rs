use model::abstract_syntax_tree::syntax_tree_node::SyntaxTreeNode;
use model::abstract_syntax_tree::AbstractSyntaxTree;
use model::syntax_analysis_context::SyntaxAnalysisContext;

use crate::lexical_analysis::model::token::Token;

#[macro_use]
mod macros;

pub(crate) mod model;

#[cfg(test)]
#[macro_use]
mod tests;

mod expression;
mod statement;

pub fn get_abstract_syntax_tree(tokens: Vec<Token>) -> AbstractSyntaxTree {
    let mut abstract_syntax_tree: Vec<SyntaxTreeNode> = vec![];
    let mut syntax_analysis_context = SyntaxAnalysisContext::new(&tokens);

    while let Some(token) = syntax_analysis_context.tokens.peek() {
        match token {
            Token::EndOfFile => break,
            _ => {
                let (returned_syntax_analysis_context, syntax_tree_node) =
                    get_next_syntax_tree_node(syntax_analysis_context);
                syntax_analysis_context = returned_syntax_analysis_context;
                if let Some(syntax_tree_node) = syntax_tree_node {
                    abstract_syntax_tree.push(syntax_tree_node)
                }
            }
        }
    }

    AbstractSyntaxTree {
        abstract_syntax_tree,
        syntax_parsing_errors: syntax_analysis_context.syntax_parsing_errors,
    }
}

fn get_next_syntax_tree_node(
    mut syntax_analysis_context: SyntaxAnalysisContext,
) -> (SyntaxAnalysisContext, Option<SyntaxTreeNode>) {
    debug!("Parsing next SyntaxTreeNode.");
    match syntax_analysis_context.tokens.peek() {
        Some(token) => match token {
            Token::Let => statement::parse_let_statement(syntax_analysis_context),
            Token::Return => statement::parse_return_statement(syntax_analysis_context),
            _ => expression::get_expression_node(syntax_analysis_context),
        },
        None => (syntax_analysis_context, None),
    }
}
