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

#[template]
#[rstest(
    code_1,
    code_2,
    code_3,
    snapshot_name,
    case(
        "let divide = fn(x) { x / 0 };",
        "divide(1);",
        "let a = 25;",
        "environment_call_error_case_1"
    ),
    case(
        "let identity = fn(x) { x };",
        "identity(1 / 0);",
        "let a = 25;",
        "environment_call_error_case_2"
    ),
    case(
        "let call_divide = fn(x) { let divide = fn(y) { y / 0 }; divide(x) };",
        "call_divide(1);",
        "let a = 25;",
        "environment_call_error_case_3"
    )
)]
fn environment_call_error_cases(code_1: &str, code_2: &str, code_3: &str, snapshot_name: &str) {}

// A call which errors must still pop its environment, otherwise the erroring call's environment
// leaks and every subsequent binding lands in it.
#[apply(environment_call_error_cases)]
fn test_environment_call_error(code_1: &str, code_2: &str, code_3: &str, snapshot_name: &str) {
    let mut evaluator = crate::evaluator::Evaluator::new();

    assert_successive_environment!(evaluator, code_1, format!("{snapshot_name}_1"));
    assert_successive_environment_error!(evaluator, code_2, format!("{snapshot_name}_2"));
    assert_successive_environment!(evaluator, code_3, format!("{snapshot_name}_3"));
}
