use super::*;

#[template]
#[rstest(
    code,
    snapshot_name,
    case("foobar;", "identifier_not_found_case_1"),
    case("let is_directory = false;\nis_file", "identifier_not_found_case_2"),
    case("let a = 5;\na + b", "identifier_not_found_case_3"),
    case(
        "let double = fn(x) { x * 2 };\ndoubel(5)",
        "identifier_not_found_case_4"
    ),
    case(
        "let double = fn(x) { x * multiplier };\ndouble(5)",
        "identifier_not_found_case_5"
    ),
    case(
        "let identity = fn(x) { x };\nidentity(1);\nx",
        "identifier_not_found_case_6"
    ),
    case("if (unknown) { 1 } else { 2 };", "identifier_not_found_case_7"),
    case("let a = b;", "identifier_not_found_case_8"),
    case("return missing;", "identifier_not_found_case_9"),
    case("!undefined_boolean;", "identifier_not_found_case_10")
)]
fn identifier_not_found_cases(code: &str, snapshot_name: &str) {}

#[apply(identifier_not_found_cases)]
fn test_identifier_not_found_lexical_analysis(code: &str, snapshot_name: &str) {
    assert_lexical_analysis!(code, snapshot_name);
}

#[apply(identifier_not_found_cases)]
fn test_identifier_not_found_syntax_analysis(code: &str, snapshot_name: &str) {
    assert_syntax_analysis!(code, snapshot_name);
}

#[apply(identifier_not_found_cases)]
fn test_identifier_not_found_evaluation_error(code: &str, snapshot_name: &str) {
    assert_evaluation_error!(code, snapshot_name);
}
