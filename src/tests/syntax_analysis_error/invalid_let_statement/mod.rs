use super::*;

#[template]
#[rstest(
    code,
    snapshot_name,
    case("let sentence = ;", "invalid_let_statement_case_1"),
    case("let", "invalid_let_statement_case_2"),
    case("\tlet file;\n", "invalid_let_statement_case_3"),
    case("    let flatcase", "invalid_let_statement_case_4"),
    case("  let camelCase\n", "invalid_let_statement_case_5"),
    case("    let PascalCase\n", "invalid_let_statement_case_6"),
    case("let UPPER_CASE_SNAKE_CASE = ;\n", "invalid_let_statement_case_7"),
    case(
        "\tlet timeout_ms = 5;\n\tlet timeout_ms_copy;\n",
        "invalid_let_statement_case_8"
    )
)]
fn invalid_let_statement_cases(code: &str, snapshot_name: &str) {}

#[apply(invalid_let_statement_cases)]
fn test_invalid_let_statement_lexical_analysis(code: &str, snapshot_name: &str) {
    assert_lexical_analysis!(code, snapshot_name);
}

#[apply(invalid_let_statement_cases)]
fn test_invalid_let_statement_syntax_analysis_error(code: &str, snapshot_name: &str) {
    assert_syntax_analysis_error!(code, snapshot_name);
}
