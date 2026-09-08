use super::*;

#[template]
#[rstest(
    code,
    snapshot_name,
    case("fn(x) { x; }(5);", "call_expression_case_1"),
    case("fn(x) { return x; }(5);", "call_expression_case_2"),
    case("fn(x) { x * 2 }(5);", "call_expression_case_3"),
    case("fn(x, y) { x + y }(2, 3);", "call_expression_case_4"),
    case(
        "let add = fn(x, y) { x + y }; add(1, add(2, 3));",
        "call_expression_case_5"
    ),
    case("fn(x) { fn(y) { y } }(1)(2);", "call_expression_case_6"),
    case(
        "let curried = fn(x) { fn(y) { y } }; curried(1)(2);",
        "call_expression_case_7"
    ),
    case(
        "fn(x) { fn(y) { fn(z) { z * 2 } } }(1)(2)(3);",
        "call_expression_case_8"
    ),
    case("let a = fn() { return 5; }(); a;", "call_expression_case_9"),
    case("fn() { return 5; }() + 1;", "call_expression_case_10"),
    case(
        "let factorial = fn(n) { if (n < 2) { return 1; } return n * factorial(n - 1); }; factorial(5);",
        "call_expression_case_11"
    ),
    case(
        "let outer = fn() { return fn() { return 7; }; }; outer()();",
        "call_expression_case_12"
    ),
    case(
        "let early = fn(x) { if (x > 10) { return x; } 0; }; early(20) + early(1);",
        "call_expression_case_13"
    ),
    case(
        "let describe = fn(x) { if (x > 0) { return \"positive\"; } return \"negative\"; }; describe(1) + describe(0 - 1);",
        "call_expression_case_14"
    )
)]
fn call_expression_cases(code: &str, snapshot_name: &str) {}

#[apply(call_expression_cases)]
fn test_call_expression_lexical_analysis(code: &str, snapshot_name: &str) {
    assert_lexical_analysis!(code, snapshot_name);
}

#[apply(call_expression_cases)]
fn test_call_expression_syntax_analysis(code: &str, snapshot_name: &str) {
    assert_syntax_analysis!(code, snapshot_name);
}

#[apply(call_expression_cases)]
fn test_call_expression_evaluation(code: &str, snapshot_name: &str) {
    assert_evaluation!(code, snapshot_name);
}

#[apply(call_expression_cases)]
fn test_call_expression_environment(code: &str, snapshot_name: &str) {
    assert_environment!(code, snapshot_name);
}
