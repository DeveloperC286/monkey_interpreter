use super::*;

#[template]
#[rstest(name, case("factorial"))]
fn code_samples(name: &str) {}

fn get_code(name: &str) -> String {
    std::fs::read_to_string(format!("./src/tests/code_samples/{name}.mk")).unwrap()
}

#[apply(code_samples)]
fn test_code_samples_lexical_analysis(name: &str) {
    assert_lexical_analysis!(&get_code(name), name);
}

#[apply(code_samples)]
fn test_code_samples_syntax_analysis(name: &str) {
    assert_syntax_analysis!(&get_code(name), name);
}

#[apply(code_samples)]
fn test_code_samples_evaluator(name: &str) {
    assert_evaluator!(&get_code(name), name);
}

#[apply(code_samples)]
fn test_code_samples_environment(name: &str) {
    assert_environment!(&get_code(name), name);
}
