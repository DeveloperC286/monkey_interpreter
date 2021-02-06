macro_rules! assert_expected_returned_tokens {
    ($code:expr, $snapshot_name:expr) => {
        insta::assert_debug_snapshot!($snapshot_name, crate::lexical_analysis::get_tokens($code));
    };
}
