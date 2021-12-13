use rstest::rstest;
use rstest_reuse::{self, *};

use crate::evaluator::Evaluator;
use crate::lexical_analysis::LexicalAnalysis;
use crate::syntax_analysis::SyntaxAnalysis;

#[template]
#[rstest(
    code,
    snapshot_name,
    case(
        "let sentence = \"this is a multiple words...\\n\";",
        "let_statement_case_1"
    )
)]
fn let_statement_cases(code: &str, snapshot_name: &str) {}

#[apply(let_statement_cases)]
fn test_let_statement_lexical_analysis(code: &str, snapshot_name: &str) {
    // When
    let tokens = LexicalAnalysis::from(code).unwrap();

    // Then
    insta::assert_debug_snapshot!(format!("test_{}_lexical_analysis", snapshot_name), tokens);
}

#[apply(let_statement_cases)]
fn test_let_statement_syntax_analysis(code: &str, snapshot_name: &str) {
    // When
    let abstract_syntax_tree = SyntaxAnalysis::from(LexicalAnalysis::from(code).unwrap()).unwrap();

    // Then
    insta::assert_debug_snapshot!(
        format!("test_{}_syntax_analysis", snapshot_name),
        abstract_syntax_tree
    );
}

#[apply(let_statement_cases)]
fn test_let_statement_evaluator(code: &str, snapshot_name: &str) {
    // When
    let mut evaluator = Evaluator::new();
    let evaluation = evaluator
        .evaluate(SyntaxAnalysis::from(LexicalAnalysis::from(code).unwrap()).unwrap())
        .unwrap();

    // Then
    insta::assert_debug_snapshot!(format!("test_{}_evaluator", snapshot_name), evaluation);
}

#[apply(let_statement_cases)]
fn test_let_statement_environment(code: &str, snapshot_name: &str) {
    // When
    let mut evaluator = Evaluator::new();
    let _evaluation = evaluator
        .evaluate(SyntaxAnalysis::from(LexicalAnalysis::from(code).unwrap()).unwrap())
        .unwrap();

    // Then
    insta::assert_debug_snapshot!(format!("test_{}_environment", snapshot_name), evaluator);
}
