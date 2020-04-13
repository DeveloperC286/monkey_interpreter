use super::super::lexical_analysis::token::{Token, TokenType};
use super::abstract_syntax_tree::syntax_tree_node::SyntaxTreeNode;
use super::abstract_syntax_tree::AbstractSyntaxTree;

use super::*;
use rstest::rstest;

#[rstest(
    tokenized_let_statement,
    expected_abstract_syntax_tree,
    case(
        vec![
            Token{token_type: TokenType::LET, literal: "let".to_string()},
            Token{token_type: TokenType::IDENTIFIER, literal: "x".to_string()},
            Token{token_type: TokenType::ASSIGN, literal: "=".to_string()},
            Token{token_type: TokenType::INTEGER, literal: "5".to_string()},
            Token{token_type: TokenType::SEMI_COLON, literal: ";".to_string()},
            Token{token_type: TokenType::EOF, literal: "".to_string()},
        ],
        AbstractSyntaxTree{ program: vec![
        ]},
    ),
)]
fn test_syntax_analysis_for_let_statements(
    tokenized_let_statement: Vec<Token>,
    expected_abstract_syntax_tree: AbstractSyntaxTree,
) {
    //given
    let syntax_analysis = SyntaxAnalysis::new(tokenized_let_statement);

    //when
    let returned_abstract_syntax_tree = syntax_analysis.parse();

    //then
    assert_abstract_syntax_tree_equal(expected_abstract_syntax_tree, returned_abstract_syntax_tree);
}

fn assert_abstract_syntax_tree_equal(
    expected_abstract_syntax_tree: AbstractSyntaxTree,
    returned_abstract_syntax_tree: AbstractSyntaxTree,
) {
    assert_eq!(
        expected_abstract_syntax_tree.program.len(),
        returned_abstract_syntax_tree.program.len()
    );

    for i in 0..expected_abstract_syntax_tree.program.len() {}
}
