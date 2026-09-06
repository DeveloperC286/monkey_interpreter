use super::*;

#[template]
#[rstest(
    code,
    snapshot_name,
    case(
        "let newAdder = fn(x) { fn(y) { x + y } };\nlet addTwo = newAdder(2);\naddTwo(3);",
        "closure_case_1"
    ),
    case(
        "let newAdder = fn(x) { fn(y) { x + y } };\nlet addTwo = newAdder(2);\nlet addThree = newAdder(3);\naddTwo(10) + addThree(10);",
        "closure_case_2"
    ),
    case(
        "let makeCounter = fn() { let count = 7; fn() { count } };\nmakeCounter()();",
        "closure_case_3"
    ),
    case(
        "let x = 10;\nlet getX = fn() { x };\nlet shadowX = fn(x) { getX() };\nshadowX(99);",
        "closure_case_4"
    ),
    case(
        "let value = 5;\nlet identity = fn(value) { value };\nidentity(value + 1);",
        "closure_case_5"
    ),
    case(
        "let compose = fn(f, g) { fn(x) { f(g(x)) } };\nlet addOne = fn(x) { x + 1 };\nlet double = fn(x) { x * 2 };\ncompose(addOne, double)(5);",
        "closure_case_6"
    ),
    case(
        "let sum = fn(n) { if (n == 0) { 0 } else { n + sum(n - 1) } };\nsum(5);",
        "closure_case_7"
    ),
    case(
        "let getHidden = fn() { hidden };\nlet caller = fn() { let hidden = 5; getHidden() };\ncaller();",
        "closure_case_8"
    )
)]
fn closure_cases(code: &str, snapshot_name: &str) {}

#[apply(closure_cases)]
fn test_closure_lexical_analysis(code: &str, snapshot_name: &str) {
    assert_lexical_analysis!(code, snapshot_name);
}

#[apply(closure_cases)]
fn test_closure_syntax_analysis(code: &str, snapshot_name: &str) {
    assert_syntax_analysis!(code, snapshot_name);
}

#[apply(closure_cases)]
fn test_closure_evaluation(code: &str, snapshot_name: &str) {
    assert_evaluation!(code, snapshot_name);
}

#[apply(closure_cases)]
fn test_closure_environment(code: &str, snapshot_name: &str) {
    assert_environment!(code, snapshot_name);
}
