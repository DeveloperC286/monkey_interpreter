use rstest::rstest;

#[macro_use]
pub(super) mod macros;

#[rstest(code, snapshot_name, case("", "test_evaluator_input_edgecases_case1"))]
fn test_evaluator_input_edgecases(code: &str, snapshot_name: &str) {
    assert_expected_returned_object!(code, snapshot_name);
}
