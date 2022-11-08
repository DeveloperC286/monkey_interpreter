use super::*;

#[template]
#[rstest(
    code_1,
    code_2,
    code_3,
    snapshot_name,
    case("let a = 25;\nlet b = 91;", "let a = a *a;", "b", "environment_case_1")
)]
fn environment_cases(code_1: &str, code_2: &str, code_3: &str, snapshot_name: &str) {}

#[apply(environment_cases)]
fn test_environment(code_1: &str, code_2: &str, code_3: &str, snapshot_name: &str) {
    let mut evaluator = crate::evaluator::Evaluator::new();

    assert_successive_environment!(evaluator, code_1, format!("{snapshot_name}_1"));
    assert_successive_environment!(evaluator, code_2, format!("{snapshot_name}_2"));
    assert_successive_environment!(evaluator, code_3, format!("{snapshot_name}_3"));
}
