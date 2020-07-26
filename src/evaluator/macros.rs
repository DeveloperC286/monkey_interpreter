#[allow(unused_macros)]
macro_rules! assert_expected_returned_object {
    ($abstract_syntax_tree:expr, $snapshot_name:expr) => {
        assert_json_snapshot!($snapshot_name, crate::evaluator::evaluate($abstract_syntax_tree));
    };
}