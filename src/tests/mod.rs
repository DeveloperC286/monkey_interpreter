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
    ),
    case(
        "let csv_content = \"col_1\\tcol_2\\tcol_3\\n\";",
        "let_statement_case_2"
    ),
    case("let double = fn(x) { x * 2 };", "let_statement_case_3"),
    case("let add = fn(x, y) { x + y }", "let_statement_case_4"),
    case("let a = 5;", "let_statement_case_5"),
    case("let a = 25;\nlet b = 91;", "let_statement_case_6"),
    case("let is_file = true;", "let_statement_case_7"),
    case("let is_directory = false;", "let_statement_case_8"),
    case("\tlet file = \"/tmp/temp.txt\";\n", "let_statement_case_9"),
    case("    let flatcase =  3;", "let_statement_case_10"),
    case("  let camelCase = false\n", "let_statement_case_11"),
    case("    let PascalCase=\"terrible format\";\n", "let_statement_case_12"),
    case("let UPPER_CASE_SNAKE_CASE = FALSE;\n", "let_statement_case_13"),
    case(
        "\tlet timeout_ms = 5;\n\tlet timeout_ms_copy = timeout_ms;\n",
        "let_statement_case_14"
    ),
    case(
        "let is_file = FALSE; let is_directory = !is_file;\n",
        "let_statement_case_15"
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
