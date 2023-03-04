use super::*;

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
    assert_lexical_analysis!(code, snapshot_name);
}

#[apply(let_statement_cases)]
fn test_let_statement_syntax_analysis(code: &str, snapshot_name: &str) {
    assert_syntax_analysis!(code, snapshot_name);
}

#[apply(let_statement_cases)]
fn test_let_statement_evaluation(code: &str, snapshot_name: &str) {
    assert_evaluation!(code, snapshot_name);
}

#[apply(let_statement_cases)]
fn test_let_statement_environment(code: &str, snapshot_name: &str) {
    assert_environment!(code, snapshot_name);
}
