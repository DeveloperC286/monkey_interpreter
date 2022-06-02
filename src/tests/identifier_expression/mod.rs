use super::*;

#[template]
#[rstest(
    code,
    snapshot_name,
    case(
        "let sentence = \"this is a multiple words...\\n\";\nsentence",
        "identifier_expression_case_1"
    ),
    case(
        "let csv_content = \"col_1\\tcol_2\\tcol_3\\n\";\ncsv_content",
        "identifier_expression_case_2"
    ),
    case(
        "let double = fn(x) { x * 2 };\ndouble",
        "identifier_expression_case_3"
    ),
    case("let add = fn(x, y) { x + y } add", "identifier_expression_case_4"),
    case("let a = 5;\na", "identifier_expression_case_5"),
    case("let a = 25;\nlet b = 91;\nb", "identifier_expression_case_6"),
    case("let is_file = true;\nis_file", "identifier_expression_case_7"),
    case("let is_directory = false;\nis_file", "identifier_expression_case_8"),
    case(
        "\tlet file = \"/tmp/temp.txt\";\nfile",
        "identifier_expression_case_9"
    ),
    case("    let flatcase =  3;\nflatcase", "identifier_expression_case_10"),
    case("  let camelCase = false\ncamelCase", "identifier_expression_case_11"),
    case(
        "    let PascalCase=\"terrible format\";\nPascalCase",
        "identifier_expression_case_12"
    ),
    case(
        "let UPPER_CASE_SNAKE_CASE = FALSE;\nUPPER_CASE_SNAKE_CASE",
        "identifier_expression_case_13"
    ),
    case(
        "\tlet timeout_ms = 5;\n\tlet timeout_ms_copy = timeout_ms;\ntimeout_ms_copy",
        "identifier_expression_case_14"
    ),
    case(
        "let is_file = FALSE; let is_directory = !is_file;\nis_directory",
        "identifier_expression_case_15"
    )
)]
fn identifier_expression_cases(code: &str, snapshot_name: &str) {}

#[apply(identifier_expression_cases)]
fn test_identifier_expression_lexical_analysis(code: &str, snapshot_name: &str) {
    assert_lexical_analysis!(code, snapshot_name);
}

#[apply(identifier_expression_cases)]
fn test_identifier_expression_syntax_analysis(code: &str, snapshot_name: &str) {
    assert_syntax_analysis!(code, snapshot_name);
}

#[apply(identifier_expression_cases)]
fn test_identifier_expression_evaluator(code: &str, snapshot_name: &str) {
    assert_evaluator!(code, snapshot_name);
}

#[apply(identifier_expression_cases)]
fn test_identifier_expression_environment(code: &str, snapshot_name: &str) {
    assert_environment!(code, snapshot_name);
}
